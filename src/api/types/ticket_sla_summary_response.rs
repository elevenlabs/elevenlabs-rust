pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketSlaSummaryResponse {
    pub metric: SlaMetric,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub duration_secs: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub started_at_unix_secs: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub due_at_unix_secs: i64,
}

impl TicketSlaSummaryResponse {
    pub fn builder() -> TicketSlaSummaryResponseBuilder {
        <TicketSlaSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSlaSummaryResponseBuilder {
    metric: Option<SlaMetric>,
    duration_secs: Option<i64>,
    started_at_unix_secs: Option<i64>,
    due_at_unix_secs: Option<i64>,
}

impl TicketSlaSummaryResponseBuilder {
    pub fn metric(mut self, value: SlaMetric) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn duration_secs(mut self, value: i64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn started_at_unix_secs(mut self, value: i64) -> Self {
        self.started_at_unix_secs = Some(value);
        self
    }

    pub fn due_at_unix_secs(mut self, value: i64) -> Self {
        self.due_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketSlaSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metric`](TicketSlaSummaryResponseBuilder::metric)
    /// - [`duration_secs`](TicketSlaSummaryResponseBuilder::duration_secs)
    /// - [`started_at_unix_secs`](TicketSlaSummaryResponseBuilder::started_at_unix_secs)
    /// - [`due_at_unix_secs`](TicketSlaSummaryResponseBuilder::due_at_unix_secs)
    pub fn build(self) -> Result<TicketSlaSummaryResponse, BuildError> {
        Ok(TicketSlaSummaryResponse {
            metric: self
                .metric
                .ok_or_else(|| BuildError::missing_field("metric"))?,
            duration_secs: self
                .duration_secs
                .ok_or_else(|| BuildError::missing_field("duration_secs"))?,
            started_at_unix_secs: self
                .started_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("started_at_unix_secs"))?,
            due_at_unix_secs: self
                .due_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("due_at_unix_secs"))?,
        })
    }
}
