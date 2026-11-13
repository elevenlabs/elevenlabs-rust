pub use crate::prelude::*;

/// Raw multipart/mixed response data that clients must parse. Contains two parts separated by boundary: 1) application/json part with composition_plan and song_metadata 2) audio/* part with binary audio data and Content-Disposition header
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComposeDetailedOkResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition_plan: Option<ComposeDetailedOkResponseCompositionPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_metadata: Option<SongMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<u8>>,
}

impl ComposeDetailedOkResponse {
    pub fn builder() -> ComposeDetailedOkResponseBuilder {
        <ComposeDetailedOkResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComposeDetailedOkResponseBuilder {
    composition_plan: Option<ComposeDetailedOkResponseCompositionPlan>,
    song_metadata: Option<SongMetadata>,
    audio: Option<Vec<u8>>,
}

impl ComposeDetailedOkResponseBuilder {
    pub fn composition_plan(mut self, value: ComposeDetailedOkResponseCompositionPlan) -> Self {
        self.composition_plan = Some(value);
        self
    }

    pub fn song_metadata(mut self, value: SongMetadata) -> Self {
        self.song_metadata = Some(value);
        self
    }

    pub fn audio(mut self, value: Vec<u8>) -> Self {
        self.audio = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComposeDetailedOkResponse`].
    pub fn build(self) -> Result<ComposeDetailedOkResponse, BuildError> {
        Ok(ComposeDetailedOkResponse {
            composition_plan: self.composition_plan,
            song_metadata: self.song_metadata,
            audio: self.audio,
        })
    }
}
