pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReceiveDubbingStreamMessage2 {
    pub status: DubbingStatusStatus,
}

impl ReceiveDubbingStreamMessage2 {
    pub fn builder() -> ReceiveDubbingStreamMessage2Builder {
        <ReceiveDubbingStreamMessage2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiveDubbingStreamMessage2Builder {
    status: Option<DubbingStatusStatus>,
}

impl ReceiveDubbingStreamMessage2Builder {
    pub fn status(mut self, value: DubbingStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiveDubbingStreamMessage2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ReceiveDubbingStreamMessage2Builder::status)
    pub fn build(self) -> Result<ReceiveDubbingStreamMessage2, BuildError> {
        Ok(ReceiveDubbingStreamMessage2 {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
