pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Base64FileSource {
    /// The name of the file, with its extension.
    #[serde(default)]
    pub filename: String,
    pub media_type: FileInputMimeType,
    /// The content of the file, base64-encoded.
    #[serde(default)]
    pub data: String,
}

impl Base64FileSource {
    pub fn builder() -> Base64FileSourceBuilder {
        <Base64FileSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Base64FileSourceBuilder {
    filename: Option<String>,
    media_type: Option<FileInputMimeType>,
    data: Option<String>,
}

impl Base64FileSourceBuilder {
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: FileInputMimeType) -> Self {
        self.media_type = Some(value);
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Base64FileSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filename`](Base64FileSourceBuilder::filename)
    /// - [`media_type`](Base64FileSourceBuilder::media_type)
    /// - [`data`](Base64FileSourceBuilder::data)
    pub fn build(self) -> Result<Base64FileSource, BuildError> {
        Ok(Base64FileSource {
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            media_type: self
                .media_type
                .ok_or_else(|| BuildError::missing_field("media_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
