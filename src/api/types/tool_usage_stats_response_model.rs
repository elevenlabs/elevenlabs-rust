pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolUsageStatsResponseModel {
    /// The total number of calls to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub total_calls: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub avg_latency_secs: f64,
}

impl ToolUsageStatsResponseModel {
    pub fn builder() -> ToolUsageStatsResponseModelBuilder {
        <ToolUsageStatsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolUsageStatsResponseModelBuilder {
    total_calls: Option<i64>,
    avg_latency_secs: Option<f64>,
}

impl ToolUsageStatsResponseModelBuilder {
    pub fn total_calls(mut self, value: i64) -> Self {
        self.total_calls = Some(value);
        self
    }

    pub fn avg_latency_secs(mut self, value: f64) -> Self {
        self.avg_latency_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolUsageStatsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avg_latency_secs`](ToolUsageStatsResponseModelBuilder::avg_latency_secs)
    pub fn build(self) -> Result<ToolUsageStatsResponseModel, BuildError> {
        Ok(ToolUsageStatsResponseModel {
            total_calls: self.total_calls,
            avg_latency_secs: self
                .avg_latency_secs
                .ok_or_else(|| BuildError::missing_field("avg_latency_secs"))?,
        })
    }
}
