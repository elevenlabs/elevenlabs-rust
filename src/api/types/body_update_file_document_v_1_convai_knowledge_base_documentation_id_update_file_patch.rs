pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
    /// Documentation that the agent will have access to in order to interact with users.
    #[serde(default)]
    pub file: Vec<u8>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
}

impl BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
    pub fn builder(
    ) -> BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatchBuilder {
        <BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatchBuilder {
    file: Option<Vec<u8>>,
    file_file_name: Option<String>,
}

impl BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatchBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatchBuilder::file)
    pub fn build(
        self,
    ) -> Result<BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch, BuildError>
    {
        Ok(
            BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
                file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
                file_file_name: self.file_file_name,
            },
        )
    }
}

impl BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
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
