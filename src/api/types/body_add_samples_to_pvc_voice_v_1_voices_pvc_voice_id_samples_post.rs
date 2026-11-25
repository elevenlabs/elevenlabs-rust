pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost {
    /// Audio files used to create the voice.
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub files_file_names: Option<Vec<Option<String>>>,
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
    files_file_names: Option<Vec<Option<String>>>,
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

    pub fn files_file_names(mut self, value: Vec<Option<String>>) -> Self {
        self.files_file_names = Some(value);
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
            files_file_names: self.files_file_names,
        })
    }
}

impl BodyAddSamplesToPvcVoiceV1VoicesPvcVoiceIdSamplesPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        let mut files_file_names_iter = self.files_file_names.unwrap_or_default().into_iter();
        for part_value in self.files {
            form = form.part("files", {
                let file_name = files_file_names_iter
                    .next()
                    .flatten()
                    .unwrap_or_else(|| "files".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(part_value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.remove_background_noise {
            form = form.text("remove_background_noise", value.to_string());
        }
        form
    }
}
