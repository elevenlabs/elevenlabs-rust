pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost {
    /// Audio recording of the user
    #[serde(default)]
    pub recording: Vec<u8>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub recording_file_name: Option<String>,
}

impl BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost {
    pub fn builder() -> BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPostBuilder {
        <BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPostBuilder {
    recording: Option<Vec<u8>>,
    recording_file_name: Option<String>,
}

impl BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPostBuilder {
    pub fn recording(mut self, value: Vec<u8>) -> Self {
        self.recording = Some(value);
        self
    }

    pub fn recording_file_name(mut self, value: impl Into<String>) -> Self {
        self.recording_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recording`](BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPostBuilder::recording)
    pub fn build(
        self,
    ) -> Result<BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost, BuildError> {
        Ok(BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost {
            recording: self
                .recording
                .ok_or_else(|| BuildError::missing_field("recording"))?,
            recording_file_name: self.recording_file_name,
        })
    }
}

impl BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part("recording", {
            let file_name = self
                .recording_file_name
                .unwrap_or_else(|| "recording".to_string());
            let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
            reqwest::multipart::Part::bytes(self.recording)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .expect("a guessed MIME type is valid")
        });
        form
    }
}
