pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketTagResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub archived: bool,
}

impl TicketTagResponse {
    pub fn builder() -> TicketTagResponseBuilder {
        <TicketTagResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketTagResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    archived: Option<bool>,
}

impl TicketTagResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketTagResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketTagResponseBuilder::id)
    /// - [`name`](TicketTagResponseBuilder::name)
    /// - [`archived`](TicketTagResponseBuilder::archived)
    pub fn build(self) -> Result<TicketTagResponse, BuildError> {
        Ok(TicketTagResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            archived: self
                .archived
                .ok_or_else(|| BuildError::missing_field("archived"))?,
        })
    }
}
