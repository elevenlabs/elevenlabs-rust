pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComposeDetailedOkResponseCompositionPlan {}

impl ComposeDetailedOkResponseCompositionPlan {
    pub fn builder() -> ComposeDetailedOkResponseCompositionPlanBuilder {
        <ComposeDetailedOkResponseCompositionPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComposeDetailedOkResponseCompositionPlanBuilder {}

impl ComposeDetailedOkResponseCompositionPlanBuilder {
    /// Consumes the builder and constructs a [`ComposeDetailedOkResponseCompositionPlan`].
    pub fn build(self) -> Result<ComposeDetailedOkResponseCompositionPlan, BuildError> {
        Ok(ComposeDetailedOkResponseCompositionPlan {})
    }
}
