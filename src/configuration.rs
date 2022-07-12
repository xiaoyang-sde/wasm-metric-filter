use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct MetricFilterConfiguration {
    pub response_code_details: bool,
}

impl Default for MetricFilterConfiguration {
    fn default() -> Self {
        MetricFilterConfiguration {
            response_code_details: false,
        }
    }
}
