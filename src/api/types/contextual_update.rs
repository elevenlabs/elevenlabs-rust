pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContextualUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ContextualUpdateInputItemType>,
    /// Context text to inject into the conversation without interrupting.
    pub text: String,
    /// Optional client-supplied identifier. When a newer update with the same context_id arrives, the older one supersedes it: the older update is dropped from the LLM view and visually marked as superseded in the rendered transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl ContextualUpdate {
    pub fn builder() -> ContextualUpdateBuilder {
        <ContextualUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContextualUpdateBuilder {
    r#type: Option<ContextualUpdateInputItemType>,
    text: Option<String>,
    context_id: Option<String>,
}

impl ContextualUpdateBuilder {
    pub fn r#type(mut self, value: ContextualUpdateInputItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContextualUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ContextualUpdateBuilder::text)
    pub fn build(self) -> Result<ContextualUpdate, BuildError> {
        Ok(ContextualUpdate {
            r#type: self.r#type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            context_id: self.context_id,
        })
    }
}
