pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FlushContext {
    pub context_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub flush: bool,
}

impl FlushContext {
    pub fn builder() -> FlushContextBuilder {
        <FlushContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlushContextBuilder {
    context_id: Option<String>,
    text: Option<String>,
    flush: Option<bool>,
}

impl FlushContextBuilder {
    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FlushContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_id`](FlushContextBuilder::context_id)
    /// - [`flush`](FlushContextBuilder::flush)
    pub fn build(self) -> Result<FlushContext, BuildError> {
        Ok(FlushContext {
            context_id: self
                .context_id
                .ok_or_else(|| BuildError::missing_field("context_id"))?,
            text: self.text,
            flush: self
                .flush
                .ok_or_else(|| BuildError::missing_field("flush"))?,
        })
    }
}
