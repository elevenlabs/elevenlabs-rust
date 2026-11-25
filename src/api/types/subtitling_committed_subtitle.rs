pub use crate::prelude::*;

/// A finished cue, and the last message with its `cue_index`. Each cue is committed
/// before any message for the next, so at most one cue is open at a time, and a session
/// that ends normally commits every cue.
///
/// The cue keeps the text its partials showed, except that its last word can move to
/// the next cue: when the text after it starts with punctuation, such as ",", that has
/// no room in this cue, the word moves with it so the next cue doesn't start with it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingCommittedSubtitle {
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
    /// When to hide the cue, in milliseconds since the session_started message.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub end_ms: i64,
}

impl SubtitlingCommittedSubtitle {
    pub fn builder() -> SubtitlingCommittedSubtitleBuilder {
        <SubtitlingCommittedSubtitleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingCommittedSubtitleBuilder {
    cue_index: Option<i64>,
    lines: Option<Vec<String>>,
    start_ms: Option<i64>,
    end_ms: Option<i64>,
}

impl SubtitlingCommittedSubtitleBuilder {
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

    pub fn end_ms(mut self, value: i64) -> Self {
        self.end_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingCommittedSubtitle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cue_index`](SubtitlingCommittedSubtitleBuilder::cue_index)
    /// - [`lines`](SubtitlingCommittedSubtitleBuilder::lines)
    /// - [`start_ms`](SubtitlingCommittedSubtitleBuilder::start_ms)
    /// - [`end_ms`](SubtitlingCommittedSubtitleBuilder::end_ms)
    pub fn build(self) -> Result<SubtitlingCommittedSubtitle, BuildError> {
        Ok(SubtitlingCommittedSubtitle {
            cue_index: self
                .cue_index
                .ok_or_else(|| BuildError::missing_field("cue_index"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
            start_ms: self
                .start_ms
                .ok_or_else(|| BuildError::missing_field("start_ms"))?,
            end_ms: self
                .end_ms
                .ok_or_else(|| BuildError::missing_field("end_ms"))?,
        })
    }
}
