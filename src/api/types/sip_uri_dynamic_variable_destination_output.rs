pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipUriDynamicVariableDestinationOutput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl SipUriDynamicVariableDestinationOutput {
    pub fn builder() -> SipUriDynamicVariableDestinationOutputBuilder {
        <SipUriDynamicVariableDestinationOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipUriDynamicVariableDestinationOutputBuilder {
    destination: Option<String>,
}

impl SipUriDynamicVariableDestinationOutputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipUriDynamicVariableDestinationOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](SipUriDynamicVariableDestinationOutputBuilder::destination)
    pub fn build(self) -> Result<SipUriDynamicVariableDestinationOutput, BuildError> {
        Ok(SipUriDynamicVariableDestinationOutput {
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
