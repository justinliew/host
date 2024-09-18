build:
	cargo component build --manifest-path=guest_rust_trace/Cargo.toml
	cp guest_rust_trace/target/wasm32-wasip1/debug/guest_rust_trace.wasm .
	cargo run -- compile guest_rust_trace.wasm guest_rust_trace.cwasm
	cargo run -- run guest_rust_trace.cwasm

clean:
	cargo clean --manifest-path=guest_rust_trace/Cargo.toml
	cargo clean