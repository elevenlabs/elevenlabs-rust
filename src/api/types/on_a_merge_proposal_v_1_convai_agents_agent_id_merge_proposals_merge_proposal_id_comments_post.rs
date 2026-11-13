pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPost {
    /// The comment text. Markdown is supported.
    #[serde(default)]
    pub body: String,
}

impl OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPost {
    pub fn builder(
    ) -> OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPostBuilder {
        <OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPostBuilder {
    body: Option<String>,
}

impl OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPostBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPostBuilder::body)
    pub fn build(
        self,
    ) -> Result<
        OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPost,
        BuildError,
    > {
        Ok(
            OnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPost {
                body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            },
        )
    }
}
