pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTicketSlaPolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<TicketSlaPolicyTargetInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl UpdateTicketSlaPolicyRequest {
    pub fn builder() -> UpdateTicketSlaPolicyRequestBuilder {
        <UpdateTicketSlaPolicyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTicketSlaPolicyRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    targets: Option<Vec<TicketSlaPolicyTargetInput>>,
    archived: Option<bool>,
}

impl UpdateTicketSlaPolicyRequestBuilder {
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

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTicketSlaPolicyRequest`].
    pub fn build(self) -> Result<UpdateTicketSlaPolicyRequest, BuildError> {
        Ok(UpdateTicketSlaPolicyRequest {
            name: self.name,
            description: self.description,
            targets: self.targets,
            archived: self.archived,
        })
    }
}
