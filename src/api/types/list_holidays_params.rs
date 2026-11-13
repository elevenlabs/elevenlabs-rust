pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListHolidaysParams {}

impl ListHolidaysParams {
    pub fn builder() -> ListHolidaysParamsBuilder {
        <ListHolidaysParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListHolidaysParamsBuilder {}

impl ListHolidaysParamsBuilder {
    /// Consumes the builder and constructs a [`ListHolidaysParams`].
    pub fn build(self) -> Result<ListHolidaysParams, BuildError> {
        Ok(ListHolidaysParams {})
    }
}
