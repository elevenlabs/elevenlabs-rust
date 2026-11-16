pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
    /// Documentation that the agent will have access to in order to interact with users.
    #[serde(default)]
    pub file: Vec<u8>,
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
}

impl BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatchBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
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
            },
        )
    }
}

impl BodyUpdateFileDocumentV1ConvaiKnowledgeBaseDocumentationIdUpdateFilePatch {
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
