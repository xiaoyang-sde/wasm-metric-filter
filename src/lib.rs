use crate::root::MetricFilterRoot;

use proxy_wasm::traits::RootContext;
use proxy_wasm::types::LogLevel;

mod configuration;
mod http;
mod root;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MetricFilterRoot::default())
    });
}}
