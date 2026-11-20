pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserMessageInputItem {
    /// The parts of the message: at most one text part and up to 5 file parts.
    #[serde(default)]
    pub content: Vec<ContentItem>,
}

impl UserMessageInputItem {
    pub fn builder() -> UserMessageInputItemBuilder {
        <UserMessageInputItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserMessageInputItemBuilder {
    content: Option<Vec<ContentItem>>,
}

impl UserMessageInputItemBuilder {
    pub fn content(mut self, value: Vec<ContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserMessageInputItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](UserMessageInputItemBuilder::content)
    pub fn build(self) -> Result<UserMessageInputItem, BuildError> {
        Ok(UserMessageInputItem {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
