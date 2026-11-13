pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetTestsSummariesByIdsResponseModel {
    /// Dictionary mapping test IDs to their summary information
    #[serde(default)]
    pub tests: std::collections::HashMap<String, UnitTestSummaryResponseModel>,
}

impl GetTestsSummariesByIdsResponseModel {
    pub fn builder() -> GetTestsSummariesByIdsResponseModelBuilder {
        <GetTestsSummariesByIdsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTestsSummariesByIdsResponseModelBuilder {
    tests: Option<std::collections::HashMap<String, UnitTestSummaryResponseModel>>,
}

impl GetTestsSummariesByIdsResponseModelBuilder {
    pub fn tests(
        mut self,
        value: std::collections::HashMap<String, UnitTestSummaryResponseModel>,
    ) -> Self {
        self.tests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTestsSummariesByIdsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tests`](GetTestsSummariesByIdsResponseModelBuilder::tests)
    pub fn build(self) -> Result<GetTestsSummariesByIdsResponseModel, BuildError> {
        Ok(GetTestsSummariesByIdsResponseModel {
            tests: self
                .tests
                .ok_or_else(|| BuildError::missing_field("tests"))?,
        })
    }
}
