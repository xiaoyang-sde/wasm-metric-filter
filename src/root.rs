use crate::configuration::MetricFilterConfiguration;
use crate::http::MetricFilter;

use proxy_wasm::hostcalls::log;
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::{ContextType, LogLevel};
use serde_json::from_slice;

#[derive(Default)]
pub struct MetricFilterRoot {
    pub configuration: MetricFilterConfiguration,
}

impl Context for MetricFilterRoot {}

impl RootContext for MetricFilterRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        let raw_configuration = match self.get_plugin_configuration() {
            Some(configuration) => configuration,
            None => {
                log(LogLevel::Info, "failed to get the configuration").ok();
                return false;
            }
        };

        self.configuration = match from_slice::<MetricFilterConfiguration>(&raw_configuration) {
            Ok(configuration) => configuration,
            Err(error) => {
                log(
                    LogLevel::Info,
                    &format!("failed to parse the configuration: {}", error,),
                )
                .ok();
                return false;
            }
        };

        true
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MetricFilter {
            configuration: self.configuration,
        }))
    }
}
