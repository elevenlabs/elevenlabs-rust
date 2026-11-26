pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketSlaPolicyTargetOutput {
    pub metric: SlaMetric,
    pub priority: Option<TicketPriority>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub duration_secs: i64,
}

impl TicketSlaPolicyTargetOutput {
    pub fn builder() -> TicketSlaPolicyTargetOutputBuilder {
        <TicketSlaPolicyTargetOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSlaPolicyTargetOutputBuilder {
    metric: Option<SlaMetric>,
    priority: Option<TicketPriority>,
    duration_secs: Option<i64>,
}

impl TicketSlaPolicyTargetOutputBuilder {
    pub fn metric(mut self, value: SlaMetric) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn priority(mut self, value: TicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn duration_secs(mut self, value: i64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketSlaPolicyTargetOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metric`](TicketSlaPolicyTargetOutputBuilder::metric)
    /// - [`duration_secs`](TicketSlaPolicyTargetOutputBuilder::duration_secs)
    pub fn build(self) -> Result<TicketSlaPolicyTargetOutput, BuildError> {
        Ok(TicketSlaPolicyTargetOutput {
            metric: self
                .metric
                .ok_or_else(|| BuildError::missing_field("metric"))?,
            priority: self.priority,
            duration_secs: self
                .duration_secs
                .ok_or_else(|| BuildError::missing_field("duration_secs"))?,
        })
    }
}
