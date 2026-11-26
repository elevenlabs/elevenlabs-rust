pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketRequesterResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

impl TicketRequesterResponse {
    pub fn builder() -> TicketRequesterResponseBuilder {
        <TicketRequesterResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketRequesterResponseBuilder {
    id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    full_name: Option<String>,
    email: Option<String>,
}

impl TicketRequesterResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketRequesterResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TicketRequesterResponseBuilder::id)
    pub fn build(self) -> Result<TicketRequesterResponse, BuildError> {
        Ok(TicketRequesterResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            full_name: self.full_name,
            email: self.email,
        })
    }
}
