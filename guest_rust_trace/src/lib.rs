#[allow(warnings)]
mod bindings;

use std::str;

use bindings::exports::fastly::varnish::trace_hooks::{Guest};
use bindings::fastly::varnish::types;

struct Component;

fn host_log(msg: &str) {
    bindings::fastly::varnish::trace_log::log(msg, "cmcd", "yG11pE2Fk9iJcfmJOy1rt6")
}

fn host_get(header: &str, res: &types::ReqResource) -> Option<Vec<Vec<u8>>> {
    bindings::fastly::varnish::types::ReqResource::get(res, header)
}

#[allow(unused)]
fn host_get_header_names(res: &types::ReqResource) -> Vec<Vec<u8>> {
    bindings::fastly::varnish::types::ReqResource::get_header_names(res)
}

#[allow(unused)]
fn host_get_service_id(res: &types::ReqResource) -> String {
    bindings::fastly::varnish::types::ReqResource::get_service_id(res)
}

#[allow(unused)]
fn host_try_pop() -> bool {
    bindings::fastly::varnish::queue::try_pop(60)
}

impl Guest for Component {
    fn enter() {
        host_log("entered");
        while host_try_pop() {
            host_log("running");
        }
       host_log("done");
    }
}

bindings::export!(Component with_types_in bindings);
