pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketSlaPolicyResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub targets: Vec<TicketSlaPolicyTargetOutput>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub updated_at_unix_secs: i64,
}

impl TicketSlaPolicyResponse {
    pub fn builder() -> TicketSlaPolicyResponseBuilder {
        <TicketSlaPolicyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSlaPolicyResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    targets: Option<Vec<TicketSlaPolicyTargetOutput>>,
    archived: Option<bool>,
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
}

impl TicketSlaPolicyResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn targets(mut self, value: Vec<TicketSlaPolicyTargetOutput>) -> Self {
        self.targets = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketSlaPolicyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketSlaPolicyResponseBuilder::id)
    /// - [`name`](TicketSlaPolicyResponseBuilder::name)
    /// - [`targets`](TicketSlaPolicyResponseBuilder::targets)
    /// - [`archived`](TicketSlaPolicyResponseBuilder::archived)
    /// - [`created_at_unix_secs`](TicketSlaPolicyResponseBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](TicketSlaPolicyResponseBuilder::updated_at_unix_secs)
    pub fn build(self) -> Result<TicketSlaPolicyResponse, BuildError> {
        Ok(TicketSlaPolicyResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            targets: self
                .targets
                .ok_or_else(|| BuildError::missing_field("targets"))?,
            archived: self
                .archived
                .ok_or_else(|| BuildError::missing_field("archived"))?,
            created_at_unix_secs: self
                .created_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self
                .updated_at_unix_secs
                .ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
        })
    }
}
