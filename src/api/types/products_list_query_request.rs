pub use crate::prelude::*;

/// Query parameters for Get All Products
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProductsListQueryRequest {
    /// Only products filed under the group with this label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Only active (`true`) or inactive (`false`) products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ProductsListQueryRequest {
    pub fn builder() -> ProductsListQueryRequestBuilder {
        <ProductsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductsListQueryRequestBuilder {
    group: Option<String>,
    is_active: Option<bool>,
}

impl ProductsListQueryRequestBuilder {
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductsListQueryRequest`].
    pub fn build(self) -> Result<ProductsListQueryRequest, BuildError> {
        Ok(ProductsListQueryRequest {
            group: self.group,
            is_active: self.is_active,
        })
    }
}
