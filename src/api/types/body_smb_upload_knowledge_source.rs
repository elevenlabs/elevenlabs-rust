pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodySmbUploadKnowledgeSource {
    /// The document the receptionist should learn from.
    #[serde(default)]
    pub file: Vec<u8>,
}

impl BodySmbUploadKnowledgeSource {
    pub fn builder() -> BodySmbUploadKnowledgeSourceBuilder {
        <BodySmbUploadKnowledgeSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySmbUploadKnowledgeSourceBuilder {
    file: Option<Vec<u8>>,
}

impl BodySmbUploadKnowledgeSourceBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySmbUploadKnowledgeSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodySmbUploadKnowledgeSourceBuilder::file)
    pub fn build(self) -> Result<BodySmbUploadKnowledgeSource, BuildError> {
        Ok(BodySmbUploadKnowledgeSource {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}

impl BodySmbUploadKnowledgeSource {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file)
                .file_name("file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form
    }
}
