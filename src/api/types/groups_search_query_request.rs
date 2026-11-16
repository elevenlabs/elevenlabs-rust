pub use crate::prelude::*;

/// Query parameters for Search user group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupsSearchQueryRequest {
    /// Name of the target group.
    pub name: String,
}

impl GroupsSearchQueryRequest {
    pub fn builder() -> GroupsSearchQueryRequestBuilder {
        <GroupsSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsSearchQueryRequestBuilder {
    name: Option<String>,
}

impl GroupsSearchQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GroupsSearchQueryRequestBuilder::name)
    pub fn build(self) -> Result<GroupsSearchQueryRequest, BuildError> {
        Ok(GroupsSearchQueryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
