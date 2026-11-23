pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MultimodalMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MultimodalMessageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MultimodalMessageText>,
    /// Deprecated: single attachment, kept for clients that predate `files`. Ignored when `files` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    /// Files attached to this message, at most 5. Each file must already have been uploaded via the conversation file upload endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FilesItem>>,
}

impl MultimodalMessage {
    pub fn builder() -> MultimodalMessageBuilder {
        <MultimodalMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MultimodalMessageBuilder {
    r#type: Option<MultimodalMessageType>,
    text: Option<MultimodalMessageText>,
    file: Option<File>,
    files: Option<Vec<FilesItem>>,
}

impl MultimodalMessageBuilder {
    pub fn r#type(mut self, value: MultimodalMessageType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn text(mut self, value: MultimodalMessageText) -> Self {
        self.text = Some(value);
        self
    }

    pub fn file(mut self, value: File) -> Self {
        self.file = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<FilesItem>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MultimodalMessage`].
    pub fn build(self) -> Result<MultimodalMessage, BuildError> {
        Ok(MultimodalMessage {
            r#type: self.r#type,
            text: self.text,
            file: self.file,
            files: self.files,
        })
    }
}
