pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTicketTagRequest {
    #[serde(default)]
    pub name: String,
}

impl CreateTicketTagRequest {
    pub fn builder() -> CreateTicketTagRequestBuilder {
        <CreateTicketTagRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTicketTagRequestBuilder {
    name: Option<String>,
}

impl CreateTicketTagRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTicketTagRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateTicketTagRequestBuilder::name)
    pub fn build(self) -> Result<CreateTicketTagRequest, BuildError> {
        Ok(CreateTicketTagRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
