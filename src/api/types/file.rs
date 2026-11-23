pub use crate::prelude::*;

/// Deprecated: single attachment, kept for clients that predate `files`. Ignored when `files` is set.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FileType>,
    #[serde(default)]
    pub file_id: String,
}

impl File {
    pub fn builder() -> FileBuilder {
        <FileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileBuilder {
    r#type: Option<FileType>,
    file_id: Option<String>,
}

impl FileBuilder {
    pub fn r#type(mut self, value: FileType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`File`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](FileBuilder::file_id)
    pub fn build(self) -> Result<File, BuildError> {
        Ok(File {
            r#type: self.r#type,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
