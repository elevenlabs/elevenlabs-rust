pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserTranscriptUserTranscriptItem {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub content: String,
}

impl UserTranscriptUserTranscriptItem {
    pub fn builder() -> UserTranscriptUserTranscriptItemBuilder {
        <UserTranscriptUserTranscriptItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserTranscriptUserTranscriptItemBuilder {
    role: Option<String>,
    content: Option<String>,
}

impl UserTranscriptUserTranscriptItemBuilder {
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserTranscriptUserTranscriptItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](UserTranscriptUserTranscriptItemBuilder::role)
    /// - [`content`](UserTranscriptUserTranscriptItemBuilder::content)
    pub fn build(self) -> Result<UserTranscriptUserTranscriptItem, BuildError> {
        Ok(UserTranscriptUserTranscriptItem {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
