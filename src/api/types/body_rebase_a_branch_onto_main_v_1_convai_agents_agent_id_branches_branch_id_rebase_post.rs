pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {}

impl BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {
    pub fn builder(
    ) -> BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {
        <BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {}

impl BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {
    /// Consumes the builder and constructs a [`BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost`].
    pub fn build(
        self,
    ) -> Result<BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost, BuildError>
    {
        Ok(BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {})
    }
}
