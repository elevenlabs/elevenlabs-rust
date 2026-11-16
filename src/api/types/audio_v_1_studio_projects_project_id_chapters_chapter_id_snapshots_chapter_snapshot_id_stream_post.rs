pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPost {
    /// Whether to convert the audio to mpeg format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_to_mpeg: Option<bool>,
}

impl AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPost {
    pub fn builder(
    ) -> AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPostBuilder
    {
        <AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPostBuilder
{
    convert_to_mpeg: Option<bool>,
}

impl AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPostBuilder {
    pub fn convert_to_mpeg(mut self, value: bool) -> Self {
        self.convert_to_mpeg = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPost`].
    pub fn build(
        self,
    ) -> Result<
        AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPost,
        BuildError,
    > {
        Ok(
            AudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPost {
                convert_to_mpeg: self.convert_to_mpeg,
            },
        )
    }
}
