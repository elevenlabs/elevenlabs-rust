pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinalAudioForTurn {
    /// Signals the end of audio for a turn. Emitted after each explicit flush, new_turn/voice-switch boundary, and on connection close. All of the turn's audio, including any bytes buffered by streaming codecs (MP3, Opus, etc.), is delivered before this marker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final_audio_for_turn: Option<bool>,
}

impl FinalAudioForTurn {
    pub fn builder() -> FinalAudioForTurnBuilder {
        <FinalAudioForTurnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalAudioForTurnBuilder {
    is_final_audio_for_turn: Option<bool>,
}

impl FinalAudioForTurnBuilder {
    pub fn is_final_audio_for_turn(mut self, value: bool) -> Self {
        self.is_final_audio_for_turn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinalAudioForTurn`].
    pub fn build(self) -> Result<FinalAudioForTurn, BuildError> {
        Ok(FinalAudioForTurn {
            is_final_audio_for_turn: self.is_final_audio_for_turn,
        })
    }
}
