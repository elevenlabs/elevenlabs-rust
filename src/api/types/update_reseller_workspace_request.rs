pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateResellerWorkspaceRequest {
    /// Replaces the workspace's whole metadata map; send `{}` to clear it. Omitted leaves it unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl UpdateResellerWorkspaceRequest {
    pub fn builder() -> UpdateResellerWorkspaceRequestBuilder {
        <UpdateResellerWorkspaceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateResellerWorkspaceRequestBuilder {
    metadata: Option<std::collections::HashMap<String, String>>,
}

impl UpdateResellerWorkspaceRequestBuilder {
    pub fn metadata(mut self, value: std::collections::HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateResellerWorkspaceRequest`].
    pub fn build(self) -> Result<UpdateResellerWorkspaceRequest, BuildError> {
        Ok(UpdateResellerWorkspaceRequest {
            metadata: self.metadata,
        })
    }
}
