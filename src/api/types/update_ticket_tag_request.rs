pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTicketTagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl UpdateTicketTagRequest {
    pub fn builder() -> UpdateTicketTagRequestBuilder {
        <UpdateTicketTagRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTicketTagRequestBuilder {
    name: Option<String>,
    archived: Option<bool>,
}

impl UpdateTicketTagRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTicketTagRequest`].
    pub fn build(self) -> Result<UpdateTicketTagRequest, BuildError> {
        Ok(UpdateTicketTagRequest {
            name: self.name,
            archived: self.archived,
        })
    }
}
