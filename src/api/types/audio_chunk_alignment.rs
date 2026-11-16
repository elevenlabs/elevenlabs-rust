pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioChunkAlignment {
    #[serde(default)]
    pub chars: Vec<String>,
    #[serde(default)]
    pub char_start_times_ms: Vec<i64>,
    #[serde(default)]
    pub char_durations_ms: Vec<i64>,
}

impl AudioChunkAlignment {
    pub fn builder() -> AudioChunkAlignmentBuilder {
        <AudioChunkAlignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioChunkAlignmentBuilder {
    chars: Option<Vec<String>>,
    char_start_times_ms: Option<Vec<i64>>,
    char_durations_ms: Option<Vec<i64>>,
}

impl AudioChunkAlignmentBuilder {
    pub fn chars(mut self, value: Vec<String>) -> Self {
        self.chars = Some(value);
        self
    }

    pub fn char_start_times_ms(mut self, value: Vec<i64>) -> Self {
        self.char_start_times_ms = Some(value);
        self
    }

    pub fn char_durations_ms(mut self, value: Vec<i64>) -> Self {
        self.char_durations_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioChunkAlignment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chars`](AudioChunkAlignmentBuilder::chars)
    /// - [`char_start_times_ms`](AudioChunkAlignmentBuilder::char_start_times_ms)
    /// - [`char_durations_ms`](AudioChunkAlignmentBuilder::char_durations_ms)
    pub fn build(self) -> Result<AudioChunkAlignment, BuildError> {
        Ok(AudioChunkAlignment {
            chars: self
                .chars
                .ok_or_else(|| BuildError::missing_field("chars"))?,
            char_start_times_ms: self
                .char_start_times_ms
                .ok_or_else(|| BuildError::missing_field("char_start_times_ms"))?,
            char_durations_ms: self
                .char_durations_ms
                .ok_or_else(|| BuildError::missing_field("char_durations_ms"))?,
        })
    }
}
