pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileIdSource {
    /// The ID that `POST /v1/convai/conversations/{conversation_id}/files` returned for this conversation.
    #[serde(default)]
    pub file_id: String,
}

impl FileIdSource {
    pub fn builder() -> FileIdSourceBuilder {
        <FileIdSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileIdSourceBuilder {
    file_id: Option<String>,
}

impl FileIdSourceBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileIdSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](FileIdSourceBuilder::file_id)
    pub fn build(self) -> Result<FileIdSource, BuildError> {
        Ok(FileIdSource {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
