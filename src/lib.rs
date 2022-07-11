use proxy_wasm::hostcalls::{define_metric, increment_metric, log};
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::{Action, ContextType, LogLevel, MetricType};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MetricFilterRoot)
    });
}}

struct MetricFilterRoot;

impl Context for MetricFilterRoot {}

impl RootContext for MetricFilterRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MetricFilter))
    }
}

struct MetricFilter;

impl Context for MetricFilter {}

impl HttpContext for MetricFilter {
    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        let status = match self.get_http_response_header(":status") {
            Some(status) => status,
            None => return Action::Continue,
        };

        let response_code_details = match self.get_http_response_header("x-response-code-details") {
            Some(response_code_details) => response_code_details,
            None => return Action::Continue,
        };

        log(
            LogLevel::Info,
            &format!(
                "status: {}, response_code_details: {}",
                status, response_code_details
            ),
        )
        .ok();

        let metric_name = format!("upstream_rq_{}_{}", status, response_code_details);
        let metric_id = match define_metric(MetricType::Counter, &metric_name) {
            Ok(metric_id) => metric_id,
            Err(_) => return Action::Continue,
        };
        increment_metric(metric_id, 1).ok();

        Action::Continue
    }
}
