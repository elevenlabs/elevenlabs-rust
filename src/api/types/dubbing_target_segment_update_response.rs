pub use crate::prelude::*;

/// The result of a target-translation edit: the updated segment and the new revision.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingTargetSegmentUpdateResponse {
    /// One segment of a target transcript: a source segment plus its translation.
    pub segment: DubbingTargetTranscriptSegment,
    /// The target's revision after this edit.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub revision: i64,
}

impl DubbingTargetSegmentUpdateResponse {
    pub fn builder() -> DubbingTargetSegmentUpdateResponseBuilder {
        <DubbingTargetSegmentUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTargetSegmentUpdateResponseBuilder {
    segment: Option<DubbingTargetTranscriptSegment>,
    revision: Option<i64>,
}

impl DubbingTargetSegmentUpdateResponseBuilder {
    pub fn segment(mut self, value: DubbingTargetTranscriptSegment) -> Self {
        self.segment = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTargetSegmentUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segment`](DubbingTargetSegmentUpdateResponseBuilder::segment)
    /// - [`revision`](DubbingTargetSegmentUpdateResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingTargetSegmentUpdateResponse, BuildError> {
        Ok(DubbingTargetSegmentUpdateResponse {
            segment: self
                .segment
                .ok_or_else(|| BuildError::missing_field("segment"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
