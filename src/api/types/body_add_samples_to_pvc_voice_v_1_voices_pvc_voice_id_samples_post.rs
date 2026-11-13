pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost {
    /// Audio files used to create the voice.
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
}

impl BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost {
    pub fn builder() -> BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPostBuilder {
        <BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPostBuilder {
    files: Option<Vec<Vec<u8>>>,
    remove_background_noise: Option<bool>,
}

impl BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPostBuilder {
    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPostBuilder::files)
    pub fn build(
        self,
    ) -> Result<BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost, BuildError> {
        Ok(BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost {
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
            remove_background_noise: self.remove_background_noise,
        })
    }
}

impl BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
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
        form
    }
}
