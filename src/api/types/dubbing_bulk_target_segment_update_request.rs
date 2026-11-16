pub use crate::prelude::*;

/// Edit several target translations at once, applied atomically (all succeed or none do).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingBulkTargetSegmentUpdateRequest {
    /// Map of segment ID to the translation edit to apply to that segment. At least one entry and at most 500.
    #[serde(default)]
    pub segments: std::collections::HashMap<String, DubbingTargetSegmentUpdateRequest>,
}

impl DubbingBulkTargetSegmentUpdateRequest {
    pub fn builder() -> DubbingBulkTargetSegmentUpdateRequestBuilder {
        <DubbingBulkTargetSegmentUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingBulkTargetSegmentUpdateRequestBuilder {
    segments: Option<std::collections::HashMap<String, DubbingTargetSegmentUpdateRequest>>,
}

impl DubbingBulkTargetSegmentUpdateRequestBuilder {
    pub fn segments(
        mut self,
        value: std::collections::HashMap<String, DubbingTargetSegmentUpdateRequest>,
    ) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingBulkTargetSegmentUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](DubbingBulkTargetSegmentUpdateRequestBuilder::segments)
    pub fn build(self) -> Result<DubbingBulkTargetSegmentUpdateRequest, BuildError> {
        Ok(DubbingBulkTargetSegmentUpdateRequest {
            segments: self
                .segments
                .ok_or_else(|| BuildError::missing_field("segments"))?,
        })
    }
}
