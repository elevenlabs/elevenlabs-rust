pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipUriDestinationOutput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl SipUriDestinationOutput {
    pub fn builder() -> SipUriDestinationOutputBuilder {
        <SipUriDestinationOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipUriDestinationOutputBuilder {
    destination: Option<String>,
}

impl SipUriDestinationOutputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipUriDestinationOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](SipUriDestinationOutputBuilder::destination)
    pub fn build(self) -> Result<SipUriDestinationOutput, BuildError> {
        Ok(SipUriDestinationOutput {
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
