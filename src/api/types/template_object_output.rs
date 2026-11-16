pub use crate::prelude::*;

/// An `object` output, delivered as `content` once completed: one output per
/// field of the schema's `properties`, each an output in its own right with
/// its own `status`. There are no dynamic fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateObjectOutput {
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
    /// One output per field, keyed by field name, each following the shape of that field's schema. Present only when `status` is `completed`. Reserved: no template produces this kind of output yet; it is published so that templates which do can be run with the same client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, TemplateOutput>>,
}

impl TemplateObjectOutput {
    pub fn builder() -> TemplateObjectOutputBuilder {
        <TemplateObjectOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateObjectOutputBuilder {
    id: Option<String>,
    status: Option<TemplateRunStatus>,
    failure_reason: Option<TemplateArrayOutputFailureReason>,
    error_message: Option<String>,
    content: Option<std::collections::HashMap<String, TemplateOutput>>,
}

impl TemplateObjectOutputBuilder {
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

    pub fn content(mut self, value: std::collections::HashMap<String, TemplateOutput>) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateObjectOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplateObjectOutputBuilder::id)
    /// - [`status`](TemplateObjectOutputBuilder::status)
    pub fn build(self) -> Result<TemplateObjectOutput, BuildError> {
        Ok(TemplateObjectOutput {
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
