pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingStatus {
    pub status: DubbingStatusStatus,
}

impl DubbingStatus {
    pub fn builder() -> DubbingStatusBuilder {
        <DubbingStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingStatusBuilder {
    status: Option<DubbingStatusStatus>,
}

impl DubbingStatusBuilder {
    pub fn status(mut self, value: DubbingStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DubbingStatusBuilder::status)
    pub fn build(self) -> Result<DubbingStatus, BuildError> {
        Ok(DubbingStatus {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
