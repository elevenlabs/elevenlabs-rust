pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddVoiceRequest {
    /// The name that identifies this voice. This will be displayed in the dropdown of the website.
    #[serde(default)]
    pub name: String,
    /// A list of file paths to audio recordings intended for voice cloning.
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    /// A description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Labels for the voice. Keys can be language, accent, gender, or age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<AddVoiceRequestLabels>,
}

impl AddVoiceRequest {
    pub fn builder() -> AddVoiceRequestBuilder {
        <AddVoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddVoiceRequestBuilder {
    name: Option<String>,
    files: Option<Vec<Vec<u8>>>,
    remove_background_noise: Option<bool>,
    description: Option<String>,
    labels: Option<AddVoiceRequestLabels>,
}

impl AddVoiceRequestBuilder {
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

    pub fn labels(mut self, value: AddVoiceRequestLabels) -> Self {
        self.labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddVoiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AddVoiceRequestBuilder::name)
    /// - [`files`](AddVoiceRequestBuilder::files)
    pub fn build(self) -> Result<AddVoiceRequest, BuildError> {
        Ok(AddVoiceRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
            remove_background_noise: self.remove_background_noise,
            description: self.description,
            labels: self.labels,
        })
    }
}

impl AddVoiceRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        for part_value in self.files {
            form = form.part(
                "files",
                reqwest::multipart::Part::bytes(part_value)
                    .file_name("files")
                    .mime_str("application/octet-stream")
                    .expect("application/octet-stream is a valid MIME type"),
            );
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
        form
    }
}
