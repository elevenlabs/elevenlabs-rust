pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost {
    /// Verification documents
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    /// Extra text to be used in the manual verification process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_text: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub files_file_names: Option<Vec<Option<String>>>,
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
    files_file_names: Option<Vec<Option<String>>>,
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

    pub fn files_file_names(mut self, value: Vec<Option<String>>) -> Self {
        self.files_file_names = Some(value);
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
                files_file_names: self.files_file_names,
            },
        )
    }
}

impl BodyRequestManualVerificationV1VoicesPvcVoiceIdVerificationPost {
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
        if let Some(value) = self.extra_text {
            form = form.text("extra_text", value.to_string());
        }
        form
    }
}
