pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryParamsJsonSchemaInput {
    #[serde(default)]
    pub properties: std::collections::HashMap<String, QueryParamsJsonSchemaInputProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl QueryParamsJsonSchemaInput {
    pub fn builder() -> QueryParamsJsonSchemaInputBuilder {
        <QueryParamsJsonSchemaInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryParamsJsonSchemaInputBuilder {
    properties: Option<std::collections::HashMap<String, QueryParamsJsonSchemaInputProperties>>,
    required: Option<Vec<String>>,
}

impl QueryParamsJsonSchemaInputBuilder {
    pub fn properties(
        mut self,
        value: std::collections::HashMap<String, QueryParamsJsonSchemaInputProperties>,
    ) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryParamsJsonSchemaInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`properties`](QueryParamsJsonSchemaInputBuilder::properties)
    pub fn build(self) -> Result<QueryParamsJsonSchemaInput, BuildError> {
        Ok(QueryParamsJsonSchemaInput {
            properties: self
                .properties
                .ok_or_else(|| BuildError::missing_field("properties"))?,
            required: self.required,
        })
    }
}
