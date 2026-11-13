pub use crate::prelude::*;

/// An `integer` output, delivered as `content` once completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TemplateIntegerOutput {
    /// The id of the generation behind this output. Pass it as a `generation` reference to use the output as an input elsewhere.
    #[serde(default)]
    pub id: String,
    /// Where a run, or one of its outputs, is in its lifecycle. The run's is rolled
    /// up from its outputs' (``aggregate_output_statuses``). A class rather than a
    /// ``Literal`` so it publishes as one named enum instead of being inlined into
    /// every kind.
    pub status: TemplateRunStatus,
    /// The category of failure. Present only when `status` is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TemplateArrayOutputFailureReason>,
    /// A human-readable description of the failure. Present only when `status` is `failed`. Failed generations are not charged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The generated integer. Present only when `status` is `completed`. Reserved: no template produces this kind of output yet; it is published so that templates which do can be run with the same client.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub content: Option<i64>,
}

impl TemplateIntegerOutput {
    pub fn builder() -> TemplateIntegerOutputBuilder {
        <TemplateIntegerOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateIntegerOutputBuilder {
    id: Option<String>,
    status: Option<TemplateRunStatus>,
    failure_reason: Option<TemplateArrayOutputFailureReason>,
    error_message: Option<String>,
    content: Option<i64>,
}

impl TemplateIntegerOutputBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: TemplateRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn failure_reason(mut self, value: TemplateArrayOutputFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn content(mut self, value: i64) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateIntegerOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplateIntegerOutputBuilder::id)
    /// - [`status`](TemplateIntegerOutputBuilder::status)
    pub fn build(self) -> Result<TemplateIntegerOutput, BuildError> {
        Ok(TemplateIntegerOutput {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            failure_reason: self.failure_reason,
            error_message: self.error_message,
            content: self.content,
        })
    }
}
