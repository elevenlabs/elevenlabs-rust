pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_location_filter: Option<bool>,
}

impl GetScheduleParams {
    pub fn builder() -> GetScheduleParamsBuilder {
        <GetScheduleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetScheduleParamsBuilder {
    include_location_filter: Option<bool>,
}

impl GetScheduleParamsBuilder {
    pub fn include_location_filter(mut self, value: bool) -> Self {
        self.include_location_filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetScheduleParams`].
    pub fn build(self) -> Result<GetScheduleParams, BuildError> {
        Ok(GetScheduleParams {
            include_location_filter: self.include_location_filter,
        })
    }
}
