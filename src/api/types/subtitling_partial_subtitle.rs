pub use crate::prelude::*;

/// The cue still being filled. Each one replaces the previous message with the same
/// `cue_index` and only adds text at the end, though line breaks may move as the cue
/// fills. A cue gets zero or more of these before its `committed_subtitle`, so it can
/// arrive already committed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingPartialSubtitle {
    /// The cue's position in the session, counting from 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub cue_index: i64,
    /// The cue's text, one entry per line.
    #[serde(default)]
    pub lines: Vec<String>,
    /// When to show the cue, in milliseconds since the session_started message. It doesn't change after the cue's first message.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub start_ms: i64,
}

impl SubtitlingPartialSubtitle {
    pub fn builder() -> SubtitlingPartialSubtitleBuilder {
        <SubtitlingPartialSubtitleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingPartialSubtitleBuilder {
    cue_index: Option<i64>,
    lines: Option<Vec<String>>,
    start_ms: Option<i64>,
}

impl SubtitlingPartialSubtitleBuilder {
    pub fn cue_index(mut self, value: i64) -> Self {
        self.cue_index = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<String>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn start_ms(mut self, value: i64) -> Self {
        self.start_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingPartialSubtitle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cue_index`](SubtitlingPartialSubtitleBuilder::cue_index)
    /// - [`lines`](SubtitlingPartialSubtitleBuilder::lines)
    /// - [`start_ms`](SubtitlingPartialSubtitleBuilder::start_ms)
    pub fn build(self) -> Result<SubtitlingPartialSubtitle, BuildError> {
        Ok(SubtitlingPartialSubtitle {
            cue_index: self
                .cue_index
                .ok_or_else(|| BuildError::missing_field("cue_index"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
            start_ms: self
                .start_ms
                .ok_or_else(|| BuildError::missing_field("start_ms"))?,
        })
    }
}
