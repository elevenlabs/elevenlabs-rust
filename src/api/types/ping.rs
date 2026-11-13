pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ping {}

impl Ping {
    pub fn builder() -> PingBuilder {
        <PingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PingBuilder {}

impl PingBuilder {
    /// Consumes the builder and constructs a [`Ping`].
    pub fn build(self) -> Result<Ping, BuildError> {
        Ok(Ping {})
    }
}
