pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost {
    pub state: MergeProposalReviewState,
    /// Optional comment to leave with the review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost {
    pub fn builder(
    ) -> ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder
    {
        <ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder
{
    state: Option<MergeProposalReviewState>,
    comment: Option<String>,
}

impl ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder {
    pub fn state(mut self, value: MergeProposalReviewState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder::state)
    pub fn build(
        self,
    ) -> Result<
        ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost,
        BuildError,
    > {
        Ok(
            ReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost {
                state: self
                    .state
                    .ok_or_else(|| BuildError::missing_field("state"))?,
                comment: self.comment,
            },
        )
    }
}
