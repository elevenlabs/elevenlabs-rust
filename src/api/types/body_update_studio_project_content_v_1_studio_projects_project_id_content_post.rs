pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPost {
    /// An optional URL from which we will extract content to initialize the Studio project. If this is set, 'from_url' and 'from_content' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
    /// An optional .epub, .pdf, .txt or similar file can be provided. If provided, we will initialize the Studio project with its content. If this is set, 'from_url' and 'from_content' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_document: Option<Vec<u8>>,
    ///
    /// An optional content to initialize the Studio project with. If this is set, 'from_url' and 'from_document' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    ///
    /// Example:
    /// [{"name": "Chapter A", "blocks": [{"sub_type": "p", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "A", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "B", "type": "tts_node"}]}, {"sub_type": "h1", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "C", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "D", "type": "tts_node"}]}]}, {"name": "Chapter B", "blocks": [{"sub_type": "p", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "E", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "F", "type": "tts_node"}]}, {"sub_type": "h2", "nodes": [{"voice_id": "6lCwbsX1yVjD49QmpkT0", "text": "G", "type": "tts_node"}, {"voice_id": "6lCwbsX1yVjD49QmpkT1", "text": "H", "type": "tts_node"}]}]}]
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_content_json: Option<String>,
    /// Whether to auto convert the Studio project to audio or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub from_document_file_name: Option<String>,
}

impl BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPost {
    pub fn builder() -> BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPostBuilder {
        <BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPostBuilder {
    from_url: Option<String>,
    from_document: Option<Vec<u8>>,
    from_content_json: Option<String>,
    auto_convert: Option<bool>,
    from_document_file_name: Option<String>,
}

impl BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPostBuilder {
    pub fn from_url(mut self, value: impl Into<String>) -> Self {
        self.from_url = Some(value.into());
        self
    }

    pub fn from_document(mut self, value: Vec<u8>) -> Self {
        self.from_document = Some(value);
        self
    }

    pub fn from_content_json(mut self, value: impl Into<String>) -> Self {
        self.from_content_json = Some(value.into());
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn from_document_file_name(mut self, value: impl Into<String>) -> Self {
        self.from_document_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPost`].
    pub fn build(
        self,
    ) -> Result<BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPost, BuildError>
    {
        Ok(
            BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPost {
                from_url: self.from_url,
                from_document: self.from_document,
                from_content_json: self.from_content_json,
                auto_convert: self.auto_convert,
                from_document_file_name: self.from_document_file_name,
            },
        )
    }
}

impl BodyUpdateStudioProjectContentV1StudioProjectsProjectIdContentPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(value) = self.from_url {
            form = form.text("from_url", value.to_string());
        }
        if let Some(value) = self.from_document {
            form = form.part("from_document", {
                let file_name = self
                    .from_document_file_name
                    .unwrap_or_else(|| "from_document".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.from_content_json {
            form = form.text("from_content_json", value.to_string());
        }
        if let Some(value) = self.auto_convert {
            form = form.text("auto_convert", value.to_string());
        }
        form
    }
}
