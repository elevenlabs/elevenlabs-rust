pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioChunkMultiAlignment {
    #[serde(default)]
    pub chars: Vec<String>,
    #[serde(default)]
    pub char_start_times_ms: Vec<i64>,
    #[serde(default)]
    pub char_durations_ms: Vec<i64>,
}

impl AudioChunkMultiAlignment {
    pub fn builder() -> AudioChunkMultiAlignmentBuilder {
        <AudioChunkMultiAlignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioChunkMultiAlignmentBuilder {
    chars: Option<Vec<String>>,
    char_start_times_ms: Option<Vec<i64>>,
    char_durations_ms: Option<Vec<i64>>,
}

impl AudioChunkMultiAlignmentBuilder {
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

    /// Consumes the builder and constructs a [`AudioChunkMultiAlignment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chars`](AudioChunkMultiAlignmentBuilder::chars)
    /// - [`char_start_times_ms`](AudioChunkMultiAlignmentBuilder::char_start_times_ms)
    /// - [`char_durations_ms`](AudioChunkMultiAlignmentBuilder::char_durations_ms)
    pub fn build(self) -> Result<AudioChunkMultiAlignment, BuildError> {
        Ok(AudioChunkMultiAlignment {
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
