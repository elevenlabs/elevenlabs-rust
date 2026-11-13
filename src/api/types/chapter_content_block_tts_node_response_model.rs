pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChapterContentBlockTtsNodeResponseModel {
    #[serde(default)]
    pub project_voice_ref_id: String,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

impl ChapterContentBlockTtsNodeResponseModel {
    pub fn builder() -> ChapterContentBlockTtsNodeResponseModelBuilder {
        <ChapterContentBlockTtsNodeResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentBlockTtsNodeResponseModelBuilder {
    project_voice_ref_id: Option<String>,
    text: Option<String>,
    voice_id: Option<String>,
}

impl ChapterContentBlockTtsNodeResponseModelBuilder {
    pub fn project_voice_ref_id(mut self, value: impl Into<String>) -> Self {
        self.project_voice_ref_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChapterContentBlockTtsNodeResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_voice_ref_id`](ChapterContentBlockTtsNodeResponseModelBuilder::project_voice_ref_id)
    /// - [`text`](ChapterContentBlockTtsNodeResponseModelBuilder::text)
    pub fn build(self) -> Result<ChapterContentBlockTtsNodeResponseModel, BuildError> {
        Ok(ChapterContentBlockTtsNodeResponseModel {
            project_voice_ref_id: self
                .project_voice_ref_id
                .ok_or_else(|| BuildError::missing_field("project_voice_ref_id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_id: self.voice_id,
        })
    }
}
