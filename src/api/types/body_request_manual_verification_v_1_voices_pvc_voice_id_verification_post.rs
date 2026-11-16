pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost {
    /// Verification documents
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    /// Extra text to be used in the manual verification process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_text: Option<String>,
}

impl BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost {
    pub fn builder() -> BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPostBuilder {
        <BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPostBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPostBuilder {
    files: Option<Vec<Vec<u8>>>,
    extra_text: Option<String>,
}

impl BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPostBuilder {
    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn extra_text(mut self, value: impl Into<String>) -> Self {
        self.extra_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPostBuilder::files)
    pub fn build(
        self,
    ) -> Result<BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost, BuildError> {
        Ok(
            BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost {
                files: self
                    .files
                    .ok_or_else(|| BuildError::missing_field("files"))?,
                extra_text: self.extra_text,
            },
        )
    }
}

impl BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost {
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
        if let Some(value) = self.extra_text {
            form = form.text("extra_text", value.to_string());
        }
        form
    }
}
