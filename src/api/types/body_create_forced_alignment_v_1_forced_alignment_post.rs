pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCreateForcedAlignmentV1ForcedAlignmentPost {
    /// The file to align. All major audio formats are supported. The file size must be less than 1GB.
    #[serde(default)]
    pub file: Vec<u8>,
    /// The text to align with the audio. The input text can be in any format, however diarization is not supported at this time.
    #[serde(default)]
    pub text: String,
}

impl BodyCreateForcedAlignmentV1ForcedAlignmentPost {
    pub fn builder() -> BodyCreateForcedAlignmentV1ForcedAlignmentPostBuilder {
        <BodyCreateForcedAlignmentV1ForcedAlignmentPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateForcedAlignmentV1ForcedAlignmentPostBuilder {
    file: Option<Vec<u8>>,
    text: Option<String>,
}

impl BodyCreateForcedAlignmentV1ForcedAlignmentPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateForcedAlignmentV1ForcedAlignmentPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyCreateForcedAlignmentV1ForcedAlignmentPostBuilder::file)
    /// - [`text`](BodyCreateForcedAlignmentV1ForcedAlignmentPostBuilder::text)
    pub fn build(self) -> Result<BodyCreateForcedAlignmentV1ForcedAlignmentPost, BuildError> {
        Ok(BodyCreateForcedAlignmentV1ForcedAlignmentPost {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}

impl BodyCreateForcedAlignmentV1ForcedAlignmentPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file)
                .file_name("file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form = form.text("text", self.text.to_string());
        form
    }
}
