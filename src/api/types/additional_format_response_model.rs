pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdditionalFormatResponseModel {
    /// The requested format.
    #[serde(default)]
    pub requested_format: String,
    /// The file extension of the additional format.
    #[serde(default)]
    pub file_extension: String,
    /// The content type of the additional format.
    #[serde(default)]
    pub content_type: String,
    /// Whether the content is base64 encoded.
    #[serde(rename = "is_base64_encoded")]
    #[serde(default)]
    pub is_base_64_encoded: bool,
    /// The content of the additional format.
    #[serde(default)]
    pub content: String,
}

impl AdditionalFormatResponseModel {
    pub fn builder() -> AdditionalFormatResponseModelBuilder {
        <AdditionalFormatResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdditionalFormatResponseModelBuilder {
    requested_format: Option<String>,
    file_extension: Option<String>,
    content_type: Option<String>,
    is_base_64_encoded: Option<bool>,
    content: Option<String>,
}

impl AdditionalFormatResponseModelBuilder {
    pub fn requested_format(mut self, value: impl Into<String>) -> Self {
        self.requested_format = Some(value.into());
        self
    }

    pub fn file_extension(mut self, value: impl Into<String>) -> Self {
        self.file_extension = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn is_base_64_encoded(mut self, value: bool) -> Self {
        self.is_base_64_encoded = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdditionalFormatResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requested_format`](AdditionalFormatResponseModelBuilder::requested_format)
    /// - [`file_extension`](AdditionalFormatResponseModelBuilder::file_extension)
    /// - [`content_type`](AdditionalFormatResponseModelBuilder::content_type)
    /// - [`is_base_64_encoded`](AdditionalFormatResponseModelBuilder::is_base_64_encoded)
    /// - [`content`](AdditionalFormatResponseModelBuilder::content)
    pub fn build(self) -> Result<AdditionalFormatResponseModel, BuildError> {
        Ok(AdditionalFormatResponseModel {
            requested_format: self
                .requested_format
                .ok_or_else(|| BuildError::missing_field("requested_format"))?,
            file_extension: self
                .file_extension
                .ok_or_else(|| BuildError::missing_field("file_extension"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            is_base_64_encoded: self
                .is_base_64_encoded
                .ok_or_else(|| BuildError::missing_field("is_base_64_encoded"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
