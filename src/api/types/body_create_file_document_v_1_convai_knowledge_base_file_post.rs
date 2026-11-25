pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePost {
    /// Documentation that the agent will have access to in order to interact with users.
    #[serde(default)]
    pub file: Vec<u8>,
    /// A custom, human-readable name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If set, the created document or folder will be placed inside the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
}

impl BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePost {
    pub fn builder() -> BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePostBuilder {
        <BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePostBuilder {
    file: Option<Vec<u8>>,
    name: Option<String>,
    parent_folder_id: Option<String>,
    file_file_name: Option<String>,
}

impl BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePostBuilder::file)
    pub fn build(self) -> Result<BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePost, BuildError> {
        Ok(BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePost {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            name: self.name,
            parent_folder_id: self.parent_folder_id,
            file_file_name: self.file_file_name,
        })
    }
}

impl BodyCreateFileDocumentV1ConvaiKnowledgeBaseFilePost {
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
        if let Some(value) = self.name {
            form = form.text("name", value.to_string());
        }
        if let Some(value) = self.parent_folder_id {
            form = form.text("parent_folder_id", value.to_string());
        }
        form
    }
}
