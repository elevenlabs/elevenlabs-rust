pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinalAudioForTurnMulti {
    /// Signals the end of audio for a turn. Emitted after each explicit flush, new_turn/voice-switch boundary, and on connection close. All of the turn's audio, including any bytes buffered by streaming codecs (MP3, Opus, etc.), is delivered before this marker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final_audio_for_turn: Option<bool>,
    /// Identifies which context this response belongs to. Only set on the multi-context endpoint; omitted (None) on the single-context endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl FinalAudioForTurnMulti {
    pub fn builder() -> FinalAudioForTurnMultiBuilder {
        <FinalAudioForTurnMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalAudioForTurnMultiBuilder {
    is_final_audio_for_turn: Option<bool>,
    context_id: Option<String>,
}

impl FinalAudioForTurnMultiBuilder {
    pub fn is_final_audio_for_turn(mut self, value: bool) -> Self {
        self.is_final_audio_for_turn = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinalAudioForTurnMulti`].
    pub fn build(self) -> Result<FinalAudioForTurnMulti, BuildError> {
        Ok(FinalAudioForTurnMulti {
            is_final_audio_for_turn: self.is_final_audio_for_turn,
            context_id: self.context_id,
        })
    }
}
