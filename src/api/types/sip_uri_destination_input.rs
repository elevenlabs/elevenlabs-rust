pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipUriDestinationInput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl SipUriDestinationInput {
    pub fn builder() -> SipUriDestinationInputBuilder {
        <SipUriDestinationInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipUriDestinationInputBuilder {
    destination: Option<String>,
}

impl SipUriDestinationInputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipUriDestinationInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](SipUriDestinationInputBuilder::destination)
    pub fn build(self) -> Result<SipUriDestinationInput, BuildError> {
        Ok(SipUriDestinationInput {
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
