pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipUriDynamicVariableDestinationInput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl SipUriDynamicVariableDestinationInput {
    pub fn builder() -> SipUriDynamicVariableDestinationInputBuilder {
        <SipUriDynamicVariableDestinationInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipUriDynamicVariableDestinationInputBuilder {
    destination: Option<String>,
}

impl SipUriDynamicVariableDestinationInputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipUriDynamicVariableDestinationInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](SipUriDynamicVariableDestinationInputBuilder::destination)
    pub fn build(self) -> Result<SipUriDynamicVariableDestinationInput, BuildError> {
        Ok(SipUriDynamicVariableDestinationInput {
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
