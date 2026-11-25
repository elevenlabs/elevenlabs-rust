pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodySmbUploadKnowledgeSource {
    /// The document the receptionist should learn from.
    #[serde(default)]
    pub file: Vec<u8>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
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
    file_file_name: Option<String>,
}

impl BodySmbUploadKnowledgeSourceBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodySmbUploadKnowledgeSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodySmbUploadKnowledgeSourceBuilder::file)
    pub fn build(self) -> Result<BodySmbUploadKnowledgeSource, BuildError> {
        Ok(BodySmbUploadKnowledgeSource {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            file_file_name: self.file_file_name,
        })
    }
}

impl BodySmbUploadKnowledgeSource {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part("file", {
            let file_name = self.file_file_name.unwrap_or_else(|| "file".to_string());
            let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
            reqwest::multipart::Part::bytes(self.file)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .expect("a guessed MIME type is valid")
        });
        form
    }
}
