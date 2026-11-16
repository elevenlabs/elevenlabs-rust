pub use crate::prelude::*;

/// List scheduled group sessions for a group service in a date range.
///
/// Group services are scheduled in advance (e.g. yoga classes, workshops) and
/// callers register against an existing session. Use this for group services;
/// use ``check_service_availability`` for appointment / rental services.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListGroupSessionsParams {}

impl ListGroupSessionsParams {
    pub fn builder() -> ListGroupSessionsParamsBuilder {
        <ListGroupSessionsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListGroupSessionsParamsBuilder {}

impl ListGroupSessionsParamsBuilder {
    /// Consumes the builder and constructs a [`ListGroupSessionsParams`].
    pub fn build(self) -> Result<ListGroupSessionsParams, BuildError> {
        Ok(ListGroupSessionsParams {})
    }
}
