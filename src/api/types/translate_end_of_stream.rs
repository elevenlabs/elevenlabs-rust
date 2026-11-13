pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateEndOfStream {}

impl TranslateEndOfStream {
    pub fn builder() -> TranslateEndOfStreamBuilder {
        <TranslateEndOfStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateEndOfStreamBuilder {}

impl TranslateEndOfStreamBuilder {
    /// Consumes the builder and constructs a [`TranslateEndOfStream`].
    pub fn build(self) -> Result<TranslateEndOfStream, BuildError> {
        Ok(TranslateEndOfStream {})
    }
}
