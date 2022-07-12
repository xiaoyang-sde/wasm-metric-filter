use proxy_wasm::hostcalls::{define_metric, increment_metric};
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::{Action, MetricType};

use crate::configuration::MetricFilterConfiguration;

#[derive(Default)]
pub struct MetricFilter {
    pub configuration: MetricFilterConfiguration,
}

impl Context for MetricFilter {}

impl HttpContext for MetricFilter {
    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        let status = match self.get_http_response_header(":status") {
            Some(status) => status,
            None => return Action::Continue,
        };

        if self.configuration.response_code_details {
            let response_code_details =
                match self.get_http_response_header("x-response-code-details") {
                    Some(response_code_details) => response_code_details,
                    None => return Action::Continue,
                };

            let metric_name = format!("upstream_rq_{}_[{}]", status, response_code_details);
            let metric_id = match define_metric(MetricType::Counter, &metric_name) {
                Ok(metric_id) => metric_id,
                Err(_) => return Action::Continue,
            };
            increment_metric(metric_id, 1).ok();
        }

        if self.configuration.response_flags {
            let response_flags = match self.get_http_response_header("x-response-flags") {
                Some(response_code_details) => response_code_details,
                None => return Action::Continue,
            };
            for response_flag in response_flags.split(',') {
                if response_flag == "-" {
                    continue;
                }

                let metric_name = format!("upstream_rq_{}_[{}]", status, response_flag);
                let metric_id = match define_metric(MetricType::Counter, &metric_name) {
                    Ok(metric_id) => metric_id,
                    Err(_) => return Action::Continue,
                };
                increment_metric(metric_id, 1).ok();
            }
        }

        Action::Continue
    }
}
