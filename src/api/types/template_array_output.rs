pub use crate::prelude::*;

/// An `array` output, delivered as `content` once completed: one output per
/// element, each an output in its own right with its own `status`. A list can
/// be arbitrarily long, so `content` is its first page; when `has_more` is
/// true, `next_cursor` leads to the rest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateArrayOutput {
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
    /// The first elements of the list, in order, each following the shape of the schema's `items`. Present only when `status` is `completed`. When `has_more` is true this is not the whole list. Reserved: no template produces this kind of output yet; it is published so that templates which do can be run with the same client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TemplateOutput>>,
    /// Pass as `cursor` to a later endpoint to fetch the elements after `content`. `null` when `content` holds the whole list, or before the output completes. Reserved: no template produces this kind of output yet; it is published so that templates which do can be run with the same client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether the list has elements beyond `content`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl TemplateArrayOutput {
    pub fn builder() -> TemplateArrayOutputBuilder {
        <TemplateArrayOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateArrayOutputBuilder {
    id: Option<String>,
    status: Option<TemplateRunStatus>,
    failure_reason: Option<TemplateArrayOutputFailureReason>,
    error_message: Option<String>,
    content: Option<Vec<TemplateOutput>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl TemplateArrayOutputBuilder {
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

    pub fn content(mut self, value: Vec<TemplateOutput>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateArrayOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplateArrayOutputBuilder::id)
    /// - [`status`](TemplateArrayOutputBuilder::status)
    pub fn build(self) -> Result<TemplateArrayOutput, BuildError> {
        Ok(TemplateArrayOutput {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            failure_reason: self.failure_reason,
            error_message: self.error_message,
            content: self.content,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
