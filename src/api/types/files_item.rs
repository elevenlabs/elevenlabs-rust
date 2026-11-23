pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FilesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FileType>,
    #[serde(default)]
    pub file_id: String,
}

impl FilesItem {
    pub fn builder() -> FilesItemBuilder {
        <FilesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilesItemBuilder {
    r#type: Option<FileType>,
    file_id: Option<String>,
}

impl FilesItemBuilder {
    pub fn r#type(mut self, value: FileType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FilesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](FilesItemBuilder::file_id)
    pub fn build(self) -> Result<FilesItem, BuildError> {
        Ok(FilesItem {
            r#type: self.r#type,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
