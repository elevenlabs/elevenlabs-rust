pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCreateMusicFinetuneV1MusicFinetunesPost {
    /// Name for the finetune (5-200 characters).
    #[serde(default)]
    pub name: String,
    /// Primary musical genre of the finetune.
    #[serde(default)]
    pub primary_genre: String,
    /// Audio files to train on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Vec<u8>>>,
    /// Tags to associate with the finetune.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Finetune visibility. Only 'private' and 'workspace' can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<BodyCreateMusicFinetuneV1MusicFinetunesPostVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<MusicModelId>,
}

impl BodyCreateMusicFinetuneV1MusicFinetunesPost {
    pub fn builder() -> BodyCreateMusicFinetuneV1MusicFinetunesPostBuilder {
        <BodyCreateMusicFinetuneV1MusicFinetunesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateMusicFinetuneV1MusicFinetunesPostBuilder {
    name: Option<String>,
    primary_genre: Option<String>,
    files: Option<Vec<Vec<u8>>>,
    tags: Option<Vec<String>>,
    visibility: Option<BodyCreateMusicFinetuneV1MusicFinetunesPostVisibility>,
    model_id: Option<MusicModelId>,
}

impl BodyCreateMusicFinetuneV1MusicFinetunesPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn primary_genre(mut self, value: impl Into<String>) -> Self {
        self.primary_genre = Some(value.into());
        self
    }

    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn visibility(
        mut self,
        value: BodyCreateMusicFinetuneV1MusicFinetunesPostVisibility,
    ) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn model_id(mut self, value: MusicModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateMusicFinetuneV1MusicFinetunesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateMusicFinetuneV1MusicFinetunesPostBuilder::name)
    /// - [`primary_genre`](BodyCreateMusicFinetuneV1MusicFinetunesPostBuilder::primary_genre)
    pub fn build(self) -> Result<BodyCreateMusicFinetuneV1MusicFinetunesPost, BuildError> {
        Ok(BodyCreateMusicFinetuneV1MusicFinetunesPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            primary_genre: self
                .primary_genre
                .ok_or_else(|| BuildError::missing_field("primary_genre"))?,
            files: self.files,
            tags: self.tags,
            visibility: self.visibility,
            model_id: self.model_id,
        })
    }
}

impl BodyCreateMusicFinetuneV1MusicFinetunesPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        form = form.text("primary_genre", self.primary_genre.to_string());
        if let Some(value) = self.files {
            for part_value in value {
                form = form.part(
                    "files",
                    reqwest::multipart::Part::bytes(part_value)
                        .file_name("files")
                        .mime_str("application/octet-stream")
                        .expect("application/octet-stream is a valid MIME type"),
                );
            }
        }
        if let Some(value) = self.tags {
            for part_value in value {
                form = form.text("tags", part_value.to_string());
            }
        }
        if let Some(value) = self.visibility {
            form = form.text("visibility", value.to_string());
        }
        if let Some(value) = self.model_id {
            form = form.text("model_id", value.to_string());
        }
        form
    }
}
