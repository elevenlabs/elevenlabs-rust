pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BinaryScoring {}

impl BinaryScoring {
    pub fn builder() -> BinaryScoringBuilder {
        <BinaryScoringBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BinaryScoringBuilder {}

impl BinaryScoringBuilder {
    /// Consumes the builder and constructs a [`BinaryScoring`].
    pub fn build(self) -> Result<BinaryScoring, BuildError> {
        Ok(BinaryScoring {})
    }
}
