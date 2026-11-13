pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPost {
    /// The language code of the media content (e.g. 'en', 'es-ES'). Must be a supported source language for some order item kind.
    #[serde(default)]
    pub declared_language: String,
    /// The media file to upload. Mutually exclusive with media_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<u8>>,
    /// A URL to fetch the media file from. Mutually exclusive with media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// The filename for URL-sourced media (e.g. 'example.mp4'). Required when using media_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url_filename: Option<String>,
    /// The MIME type for URL-sourced media (e.g. 'video/mp4'). Required when using media_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url_content_type: Option<String>,
}

impl BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPost {
    pub fn builder() -> BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPostBuilder {
        <BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPostBuilder {
    declared_language: Option<String>,
    media: Option<Vec<u8>>,
    media_url: Option<String>,
    media_url_filename: Option<String>,
    media_url_content_type: Option<String>,
}

impl BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPostBuilder {
    pub fn declared_language(mut self, value: impl Into<String>) -> Self {
        self.declared_language = Some(value.into());
        self
    }

    pub fn media(mut self, value: Vec<u8>) -> Self {
        self.media = Some(value);
        self
    }

    pub fn media_url(mut self, value: impl Into<String>) -> Self {
        self.media_url = Some(value.into());
        self
    }

    pub fn media_url_filename(mut self, value: impl Into<String>) -> Self {
        self.media_url_filename = Some(value.into());
        self
    }

    pub fn media_url_content_type(mut self, value: impl Into<String>) -> Self {
        self.media_url_content_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`declared_language`](BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPostBuilder::declared_language)
    pub fn build(self) -> Result<BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPost, BuildError> {
        Ok(BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPost {
            declared_language: self
                .declared_language
                .ok_or_else(|| BuildError::missing_field("declared_language"))?,
            media: self.media,
            media_url: self.media_url,
            media_url_filename: self.media_url_filename,
            media_url_content_type: self.media_url_content_type,
        })
    }
}

impl BodyRegisterMediaV1ProductionsOrdersOrderIdMediaPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("declared_language", self.declared_language.to_string());
        if let Some(value) = self.media {
            form = form.part(
                "media",
                reqwest::multipart::Part::bytes(value)
                    .file_name("media")
                    .mime_str("application/octet-stream")
                    .expect("application/octet-stream is a valid MIME type"),
            );
        }
        if let Some(value) = self.media_url {
            form = form.text("media_url", value.to_string());
        }
        if let Some(value) = self.media_url_filename {
            form = form.text("media_url_filename", value.to_string());
        }
        if let Some(value) = self.media_url_content_type {
            form = form.text("media_url_content_type", value.to_string());
        }
        form
    }
}
