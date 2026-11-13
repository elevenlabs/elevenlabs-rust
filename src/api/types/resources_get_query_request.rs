pub use crate::prelude::*;

/// Query parameters for Get Resource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourcesGetQueryRequest {
    /// Resource type of the target resource.
    pub resource_type: WorkspaceResourceType,
}

impl ResourcesGetQueryRequest {
    pub fn builder() -> ResourcesGetQueryRequestBuilder {
        <ResourcesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResourcesGetQueryRequestBuilder {
    resource_type: Option<WorkspaceResourceType>,
}

impl ResourcesGetQueryRequestBuilder {
    pub fn resource_type(mut self, value: WorkspaceResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResourcesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_type`](ResourcesGetQueryRequestBuilder::resource_type)
    pub fn build(self) -> Result<ResourcesGetQueryRequest, BuildError> {
        Ok(ResourcesGetQueryRequest {
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
        })
    }
}
