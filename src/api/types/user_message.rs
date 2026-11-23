pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UserMessageInputItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_medium: Option<ChatSourceMedium>,
}

impl UserMessage {
    pub fn builder() -> UserMessageBuilder {
        <UserMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserMessageBuilder {
    id: Option<String>,
    r#type: Option<UserMessageInputItemType>,
    text: Option<String>,
    user_identifier: Option<String>,
    source_medium: Option<ChatSourceMedium>,
}

impl UserMessageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UserMessageInputItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn user_identifier(mut self, value: impl Into<String>) -> Self {
        self.user_identifier = Some(value.into());
        self
    }

    pub fn source_medium(mut self, value: ChatSourceMedium) -> Self {
        self.source_medium = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserMessage`].
    pub fn build(self) -> Result<UserMessage, BuildError> {
        Ok(UserMessage {
            id: self.id,
            r#type: self.r#type,
            text: self.text,
            user_identifier: self.user_identifier,
            source_medium: self.source_medium,
        })
    }
}
