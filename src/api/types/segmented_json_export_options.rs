pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentedJsonExportOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_speakers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_timestamps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub segment_on_silence_longer_than_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub max_segment_duration_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub max_segment_chars: Option<i64>,
}

impl SegmentedJsonExportOptions {
    pub fn builder() -> SegmentedJsonExportOptionsBuilder {
        <SegmentedJsonExportOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentedJsonExportOptionsBuilder {
    include_speakers: Option<bool>,
    include_timestamps: Option<bool>,
    segment_on_silence_longer_than_s: Option<f64>,
    max_segment_duration_s: Option<f64>,
    max_segment_chars: Option<i64>,
}

impl SegmentedJsonExportOptionsBuilder {
    pub fn include_speakers(mut self, value: bool) -> Self {
        self.include_speakers = Some(value);
        self
    }

    pub fn include_timestamps(mut self, value: bool) -> Self {
        self.include_timestamps = Some(value);
        self
    }

    pub fn segment_on_silence_longer_than_s(mut self, value: f64) -> Self {
        self.segment_on_silence_longer_than_s = Some(value);
        self
    }

    pub fn max_segment_duration_s(mut self, value: f64) -> Self {
        self.max_segment_duration_s = Some(value);
        self
    }

    pub fn max_segment_chars(mut self, value: i64) -> Self {
        self.max_segment_chars = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentedJsonExportOptions`].
    pub fn build(self) -> Result<SegmentedJsonExportOptions, BuildError> {
        Ok(SegmentedJsonExportOptions {
            include_speakers: self.include_speakers,
            include_timestamps: self.include_timestamps,
            segment_on_silence_longer_than_s: self.segment_on_silence_longer_than_s,
            max_segment_duration_s: self.max_segment_duration_s,
            max_segment_chars: self.max_segment_chars,
        })
    }
}
