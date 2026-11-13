pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost {
    /// Whether to convert the audio to mpeg format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_to_mpeg: Option<bool>,
}

impl StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost {
    pub fn builder(
    ) -> StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder
    {
        <StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder
{
    convert_to_mpeg: Option<bool>,
}

impl StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder {
    pub fn convert_to_mpeg(mut self, value: bool) -> Self {
        self.convert_to_mpeg = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost`].
    pub fn build(
        self,
    ) -> Result<
        StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost,
        BuildError,
    > {
        Ok(
            StreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost {
                convert_to_mpeg: self.convert_to_mpeg,
            },
        )
    }
}
