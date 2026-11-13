pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceAnalyticsQueryResponseModel {
    #[serde(default)]
    pub columns: Vec<String>,
    #[serde(default)]
    pub column_types: Vec<ColumnTypesItem>,
    #[serde(default)]
    pub rows: Vec<Vec<Option<RowsItem>>>,
    #[serde(default)]
    pub column_units: Vec<Option<ColumnUnit>>,
}

impl WorkspaceAnalyticsQueryResponseModel {
    pub fn builder() -> WorkspaceAnalyticsQueryResponseModelBuilder {
        <WorkspaceAnalyticsQueryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceAnalyticsQueryResponseModelBuilder {
    columns: Option<Vec<String>>,
    column_types: Option<Vec<ColumnTypesItem>>,
    rows: Option<Vec<Vec<Option<RowsItem>>>>,
    column_units: Option<Vec<Option<ColumnUnit>>>,
}

impl WorkspaceAnalyticsQueryResponseModelBuilder {
    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn column_types(mut self, value: Vec<ColumnTypesItem>) -> Self {
        self.column_types = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<Vec<Option<RowsItem>>>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn column_units(mut self, value: Vec<Option<ColumnUnit>>) -> Self {
        self.column_units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceAnalyticsQueryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`columns`](WorkspaceAnalyticsQueryResponseModelBuilder::columns)
    /// - [`column_types`](WorkspaceAnalyticsQueryResponseModelBuilder::column_types)
    /// - [`rows`](WorkspaceAnalyticsQueryResponseModelBuilder::rows)
    /// - [`column_units`](WorkspaceAnalyticsQueryResponseModelBuilder::column_units)
    pub fn build(self) -> Result<WorkspaceAnalyticsQueryResponseModel, BuildError> {
        Ok(WorkspaceAnalyticsQueryResponseModel {
            columns: self
                .columns
                .ok_or_else(|| BuildError::missing_field("columns"))?,
            column_types: self
                .column_types
                .ok_or_else(|| BuildError::missing_field("column_types"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            column_units: self
                .column_units
                .ok_or_else(|| BuildError::missing_field("column_units"))?,
        })
    }
}
