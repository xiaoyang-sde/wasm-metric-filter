use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Default)]
pub struct MetricFilterConfiguration {
    pub response_code_details: bool,
    pub response_flags: bool,
}
