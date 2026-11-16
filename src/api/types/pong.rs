pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Pong {}

impl Pong {
    pub fn builder() -> PongBuilder {
        <PongBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PongBuilder {}

impl PongBuilder {
    /// Consumes the builder and constructs a [`Pong`].
    pub fn build(self) -> Result<Pong, BuildError> {
        Ok(Pong {})
    }
}
