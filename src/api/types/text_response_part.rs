pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TextResponsePart {
    #[serde(default)]
    pub text: String,
    pub r#type: AgentChatResponsePartType,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    #[serde(default)]
    pub response_id: String,
}

impl TextResponsePart {
    pub fn builder() -> TextResponsePartBuilder {
        <TextResponsePartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextResponsePartBuilder {
    text: Option<String>,
    r#type: Option<AgentChatResponsePartType>,
    event_id: Option<i64>,
    response_id: Option<String>,
}

impl TextResponsePartBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AgentChatResponsePartType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn response_id(mut self, value: impl Into<String>) -> Self {
        self.response_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextResponsePart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextResponsePartBuilder::text)
    /// - [`r#type`](TextResponsePartBuilder::type)
    /// - [`event_id`](TextResponsePartBuilder::event_id)
    /// - [`response_id`](TextResponsePartBuilder::response_id)
    pub fn build(self) -> Result<TextResponsePart, BuildError> {
        Ok(TextResponsePart {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            response_id: self
                .response_id
                .ok_or_else(|| BuildError::missing_field("response_id"))?,
        })
    }
}
