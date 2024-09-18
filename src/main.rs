use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Error};
use clap::Parser;
use wasmtime::component::{Component, Linker};
use wasmtime::{
    component::Resource, Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig,
    Store, WasmBacktraceDetails,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::fastly::varnish::types;

wasmtime::component::bindgen!({
    world: "trace",
    async: true,
});

struct TraceCtx;

struct Ctx {
    table: ResourceTable,
    wasi: WasiCtx,
    varnish: TraceCtx,
}

#[async_trait::async_trait]
impl crate::fastly::varnish::trace_log::Host for TraceCtx {
    async fn log(&mut self, msg: String, endpoint: String, sid: String) {
        println!("msg from guest is {msg}, to endpoint {endpoint} and sid {sid}");
    }
}

#[async_trait::async_trait]
impl types::Host for TraceCtx {}

// fn new(&mut self) -> wasmtime::component::Resource<docs::rpn::types::Engine> { /* ... */ }
// fn push_operand(&mut self, self_: wasmtime::component::Resource<docs::rpn::types::Engine>) { /* ... */ }

#[async_trait::async_trait]
impl crate::fastly::varnish::types::HostReqResource for TraceCtx {
    async fn get_header_names(&mut self, _res: Resource<types::ReqResource>) -> Vec<Vec<u8>> {
        vec![]
    }

    async fn get(
        &mut self,
        _res: Resource<types::ReqResource>,
        _header: String,
    ) -> Option<Vec<Vec<u8>>> {
        None
    }

    async fn get_service_id(&mut self, _res: Resource<types::ReqResource>) -> String {
        "sid".to_string()
    }

    fn drop(&mut self, _res: Resource<types::ReqResource>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::fastly::varnish::queue::Host for TraceCtx {
    async fn try_pop(&mut self, _timeout_secs: u64) -> bool {
        // this await is what is causing the issue
        tokio::time::sleep(Duration::from_secs(1)).await;
        false
    }
}

impl WasiView for Ctx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub fn my_add_to_linker<T: WasiView>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()> {
    let _ = l;

    wasmtime_wasi::add_to_linker_async(l)?;
    Ok(())
}

fn create_engine() -> Result<(Engine, Linker<Ctx>), Error> {
    let pooling_allocation_config = make_pooling_config();

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
    config.epoch_interruption(true);

    config.allocation_strategy(InstanceAllocationStrategy::Pooling(
        pooling_allocation_config,
    ));

    let engine = Engine::new(&config)?;

    let mut linker: Linker<Ctx> = Linker::new(&engine);
    Trace::add_to_linker(&mut linker, |ctx| &mut ctx.varnish)?;
    my_add_to_linker(&mut linker)?;

    Ok((engine, linker))
}

fn make_pooling_config() -> PoolingAllocationConfig {
    const MB: usize = 1 << 20;
    let mut pooling_allocation_config = PoolingAllocationConfig::default();

    // This number matches Compute production
    pooling_allocation_config.max_core_instance_size(MB);

    // Core wasm programs have 1 memory
    // This is where things get expensive. Each of these reserves virtual memory,
    pooling_allocation_config.total_memories(100);
    pooling_allocation_config.max_memories_per_module(1);

    // allow for up to 2MiB of linear memory. Wasm pages are 64k
    pooling_allocation_config.memory_pages(2 * (MB as u64) / (64 * 1024));

    // Core wasm programs have 1 table
    pooling_allocation_config.max_tables_per_module(1);

    // Some applications create a large number of functions, in particular
    // when compiled in debug mode or applications written in swift. Every
    // function can end up in the table
    pooling_allocation_config.table_elements(98765);

    // Maximum number of slots in the pooling allocator to keep "warm", or those
    // to keep around to possibly satisfy an affine allocation request or an
    // instantiation of a module previously instantiated within the pool.
    pooling_allocation_config.max_unused_warm_slots(10);

    // Use a large pool, but one smaller than the default of 1000 to avoid runnign out of virtual
    // memory space if multiple engines are spun up in a single process. We'll likely want to move
    // to the on-demand allocator eventually for most purposes; see
    // https://github.com/fastly/Viceroy/issues/255
    pooling_allocation_config.total_core_instances(100);
    pooling_allocation_config
}

#[derive(Parser, Debug)]
struct Compile {
    /// Path to the file to compile
    input: PathBuf,

    /// Where to write the compiled file
    output: PathBuf,
}

#[derive(Parser, Debug)]
struct Run {
    /// Path to the file to run
    file_name: PathBuf,

    /// Fault option string
    fault: Option<String>,
}

#[derive(Parser, Debug)]
enum Cli {
    /// Compile the specified WASM to machine code.
    Compile(Compile),
    /// Run the specified machine code.
    Run(Run),
}

async fn do_compile(c: Compile) -> Result<(), Error> {
    let (engine, linker) = create_engine()?;
    let component = Component::from_file(&engine, c.input)?;
    let _varnish_pre = linker
        .instantiate_pre(&component)
        .context("conforms to Varnish world")?;
    let serialized = component.serialize()?;
    std::fs::write(c.output, serialized)?;
    Ok(())
}

async fn do_run(r: Run) -> Result<(), Error> {
    let (engine, linker) = create_engine()?;

    let component = if r.file_name.extension().map(|e| e.to_str().unwrap()) == Some("cwasm") {
        unsafe { Component::deserialize_file(&engine, &r.file_name) }?
    } else {
        Component::from_file(&engine, &r.file_name)?
    };

    // TODO: load out of a pre-compiled shared object instead of compiling on
    // demand
    let varnish_pre = linker.instantiate_pre(&component)?;

    // all this loops
    let wasi = WasiCtxBuilder::new()
        .env("INJECT_FAULT", r.fault.unwrap_or_default())
        .build();
    let ctx = Ctx {
        table: ResourceTable::new(),
        wasi,
        varnish: TraceCtx,
    };
    let mut store = Store::new(&engine, ctx);
    store.set_epoch_deadline(10);
    store.epoch_deadline_trap();

    let (trace, _instance) = Trace::instantiate_pre(&mut store, &varnish_pre).await?;

    let ticker_stop = Arc::new(AtomicBool::new(false));

    let engine_for_ticker = engine.clone();
    let stop_for_ticker = ticker_stop.clone();
    let ticker_handle = std::thread::Builder::new()
        .name("wasm-epoch-ticker".into())
        .spawn(|| {
            let engine = engine_for_ticker;
            let stop = stop_for_ticker;
            while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(10));
                engine.increment_epoch();
            }
        })
        .expect("can spawn thread");

    // loop over this to re-enter the module
    let f = async {
        trace
            .fastly_varnish_trace_hooks()
            .call_enter(&mut store)
            .await
    };

    let () = tokio::time::timeout(Duration::from_secs(1), f)
        .await
        .context("guest timed out")?
        .context("guest trapped")?;

    ticker_stop.store(true, std::sync::atomic::Ordering::Relaxed);
    let () = ticker_handle.join().unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args {
        Cli::Compile(c) => do_compile(c).await?,
        Cli::Run(r) => do_run(r).await?,
    }
    Ok(())
}
