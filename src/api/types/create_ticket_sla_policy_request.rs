pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTicketSlaPolicyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub targets: Vec<TicketSlaPolicyTargetInput>,
}

impl CreateTicketSlaPolicyRequest {
    pub fn builder() -> CreateTicketSlaPolicyRequestBuilder {
        <CreateTicketSlaPolicyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTicketSlaPolicyRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    targets: Option<Vec<TicketSlaPolicyTargetInput>>,
}

impl CreateTicketSlaPolicyRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn targets(mut self, value: Vec<TicketSlaPolicyTargetInput>) -> Self {
        self.targets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTicketSlaPolicyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateTicketSlaPolicyRequestBuilder::name)
    /// - [`targets`](CreateTicketSlaPolicyRequestBuilder::targets)
    pub fn build(self) -> Result<CreateTicketSlaPolicyRequest, BuildError> {
        Ok(CreateTicketSlaPolicyRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            targets: self
                .targets
                .ok_or_else(|| BuildError::missing_field("targets"))?,
        })
    }
}
