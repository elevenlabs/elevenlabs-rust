pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddPronunciationDictionaryRequest {
    /// The name of the pronunciation dictionary, used for identification only.
    #[serde(default)]
    pub name: String,
    /// A lexicon .pls file which we will use to initialize the project with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    /// A description of the pronunciation dictionary, used for identification only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Should be one of 'admin', 'editor' or 'viewer'. If not provided, defaults to no access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access: Option<
        AddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostWorkspaceAccess,
    >,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
}

impl AddPronunciationDictionaryRequest {
    pub fn builder() -> AddPronunciationDictionaryRequestBuilder {
        <AddPronunciationDictionaryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPronunciationDictionaryRequestBuilder {
    name: Option<String>,
    file: Option<Vec<u8>>,
    description: Option<String>,
    workspace_access: Option<
        AddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostWorkspaceAccess,
    >,
    file_file_name: Option<String>,
}

impl AddPronunciationDictionaryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn workspace_access(
        mut self,
        value: AddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostWorkspaceAccess,
    ) -> Self {
        self.workspace_access = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddPronunciationDictionaryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AddPronunciationDictionaryRequestBuilder::name)
    pub fn build(self) -> Result<AddPronunciationDictionaryRequest, BuildError> {
        Ok(AddPronunciationDictionaryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            file: self.file,
            description: self.description,
            workspace_access: self.workspace_access,
            file_file_name: self.file_file_name,
        })
    }
}

impl AddPronunciationDictionaryRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        if let Some(value) = self.file {
            form = form.part("file", {
                let file_name = self.file_file_name.unwrap_or_else(|| "file".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.description {
            form = form.text("description", value.to_string());
        }
        if let Some(value) = self.workspace_access {
            form = form.text("workspace_access", value.to_string());
        }
        form
    }
}
