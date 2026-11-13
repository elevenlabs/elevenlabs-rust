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

    /// Consumes the builder and constructs a [`AddPronunciationDictionaryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AddPronunciationDictionaryRequestBuilder::name)
    pub fn build(self) -> Result<AddPronunciationDictionaryRequest, BuildError> {
        Ok(AddPronunciationDictionaryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            file: self.file,
            description: self.description,
            workspace_access: self.workspace_access,
        })
    }
}

impl AddPronunciationDictionaryRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        if let Some(value) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(value)
                    .file_name("file")
                    .mime_str("application/octet-stream")
                    .expect("application/octet-stream is a valid MIME type"),
            );
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
