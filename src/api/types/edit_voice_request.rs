pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditVoiceRequest {
    /// The name that identifies this voice. This will be displayed in the dropdown of the website.
    #[serde(default)]
    pub name: String,
    /// Audio files to add to the voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Vec<u8>>>,
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    /// A description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Labels for the voice. Keys can be language, accent, gender, or age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<EditVoiceRequestLabels>,
    /// Run synchronous LLM moderation over the voice name and description when they change. Has no effect unless the voice_library_metadata_moderation feature flag is enabled for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderate_metadata: Option<bool>,
}

impl EditVoiceRequest {
    pub fn builder() -> EditVoiceRequestBuilder {
        <EditVoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditVoiceRequestBuilder {
    name: Option<String>,
    files: Option<Vec<Vec<u8>>>,
    remove_background_noise: Option<bool>,
    description: Option<String>,
    labels: Option<EditVoiceRequestLabels>,
    moderate_metadata: Option<bool>,
}

impl EditVoiceRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn labels(mut self, value: EditVoiceRequestLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn moderate_metadata(mut self, value: bool) -> Self {
        self.moderate_metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditVoiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EditVoiceRequestBuilder::name)
    pub fn build(self) -> Result<EditVoiceRequest, BuildError> {
        Ok(EditVoiceRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            files: self.files,
            remove_background_noise: self.remove_background_noise,
            description: self.description,
            labels: self.labels,
            moderate_metadata: self.moderate_metadata,
        })
    }
}

impl EditVoiceRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        if let Some(value) = self.files {
            for part_value in value {
                form = form.part(
                    "files",
                    reqwest::multipart::Part::bytes(part_value)
                        .file_name("files")
                        .mime_str("application/octet-stream")
                        .expect("application/octet-stream is a valid MIME type"),
                );
            }
        }
        if let Some(value) = self.remove_background_noise {
            form = form.text("remove_background_noise", value.to_string());
        }
        if let Some(value) = self.description {
            form = form.text("description", value.to_string());
        }
        if let Some(value) = self.labels {
            form = form.text("labels", serde_json::to_string(&value).unwrap_or_default());
        }
        if let Some(value) = self.moderate_metadata {
            form = form.text("moderate_metadata", value.to_string());
        }
        form
    }
}
