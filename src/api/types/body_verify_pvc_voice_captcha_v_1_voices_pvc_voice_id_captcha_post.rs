pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost {
    /// Audio recording of the user
    #[serde(default)]
    pub recording: Vec<u8>,
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
}

impl BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPostBuilder {
    pub fn recording(mut self, value: Vec<u8>) -> Self {
        self.recording = Some(value);
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
        })
    }
}

impl BodyVerifyPvcVoiceCaptchaV1VoicesPvcVoiceIdCaptchaPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "recording",
            reqwest::multipart::Part::bytes(self.recording)
                .file_name("recording")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form
    }
}
