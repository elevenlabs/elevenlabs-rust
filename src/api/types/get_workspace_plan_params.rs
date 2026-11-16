pub use crate::prelude::*;

/// Read the workspace's current Reception plan and its hard limits.
///
/// Call this before answering any plan, pricing, trial, or limit question so
/// the assistant stops guessing (e.g. telling a paying owner they're on the
/// trial, or hallucinating concurrent-call / receptionist / voice-cloning
/// caps). The tool exposes only the shape of the plan -- a display name;
/// whether it's trial / paid / untiered; the entitlement caps (max
/// receptionists, phone numbers, active locations, team members, concurrent
/// calls, voice-cloning quota) alongside the current counts; and whether
/// there's a payment issue. It deliberately does NOT return list prices or
/// dollar amounts, nor the trial / renewal end date: pricing and renewal
/// dates are a product decision and live in the Settings > Billing plan
/// picker.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkspacePlanParams {}

impl GetWorkspacePlanParams {
    pub fn builder() -> GetWorkspacePlanParamsBuilder {
        <GetWorkspacePlanParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkspacePlanParamsBuilder {}

impl GetWorkspacePlanParamsBuilder {
    /// Consumes the builder and constructs a [`GetWorkspacePlanParams`].
    pub fn build(self) -> Result<GetWorkspacePlanParams, BuildError> {
        Ok(GetWorkspacePlanParams {})
    }
}
