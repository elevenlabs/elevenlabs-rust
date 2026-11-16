pub use crate::prelude::*;

/// Query parameters for List conversations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsListQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter conversations where any of these agents participated. Can not exceed 50 values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visited_agent_ids: Option<Vec<String>>,
    /// Filter conversations where any of these agent branches participated. Can not exceed 50 values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visited_agent_branch_ids: Option<Vec<String>>,
    /// Filter conversations where any of these procedures were triggered. Can not exceed 50 values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_procedure_ids: Option<Vec<String>>,
    /// The result of the success evaluation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_successful: Option<EvaluationResultFilter>,
    /// Unix timestamp (in seconds) to filter conversations up to this start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_start_before_unix: Option<i64>,
    /// Unix timestamp (in seconds) to filter conversations after to this start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_start_after_unix: Option<i64>,
    /// Minimum call duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_duration_min_secs: Option<i64>,
    /// Maximum call duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_duration_max_secs: Option<i64>,
    /// Maximum overall rating (1-5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_max: Option<i64>,
    /// Minimum overall rating (1-5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_min: Option<i64>,
    /// Filter conversations with user feedback comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_feedback_comment: Option<bool>,
    /// Filter conversations by the user ID who initiated them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Evaluation filters. Repeat param. Format: criteria_id:result where result is one of success|failure|unknown. Example: eval=value_framing:success
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_params: Option<Vec<String>>,
    /// Data collection filters. Repeat param. Format: id:op:value where op is one of eq|neq|gt|gte|lt|lte|in. For in, pipe-delimit values. An empty value matches conversations where the field was not collected (id:eq:), and neq with an empty value matches where it was (id:neq:). eq is exact equality. gt|gte|lt|lte require a numeric value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_params: Option<Vec<String>>,
    /// Dynamic variable filters. Repeat param. Format: name:op:value where op is one of eq|neq|gt|gte|lt|lte|in. For in, pipe-delimit values. An empty value matches conversations where the variable was not set (name:eq:), and neq with an empty value matches where it was (name:neq:). eq is exact equality. gt|gte|lt|lte require a numeric value. Names containing ':' cannot be expressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable_params: Option<Vec<String>>,
    /// A single data collection filter, for callers that can only send one scalar value per query param. Same format as data_collection_params (id:op:value); combined with it when both are given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_filter: Option<String>,
    /// A single dynamic variable filter, for callers that can only send one scalar value per query param. Same format as dynamic_variable_params (name:op:value); combined with it when both are given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable_filter: Option<String>,
    /// Data collection field IDs to include in each conversation summary. Repeat param. When omitted, data_collection_results is not returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_ids: Option<Vec<String>>,
    /// Evaluation criteria IDs to include in each conversation summary. Repeat param. When omitted, evaluation_criteria_results is not returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria_ids: Option<Vec<String>>,
    /// Filter conversations by tool names used during the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
    /// Filter conversations by tool names that had successful calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names_successful: Option<Vec<String>>,
    /// Filter conversations by tool names that had errored calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names_errored: Option<Vec<String>>,
    /// Filter conversations by IDs of tools used during the call. Use '__system_<tool_name>' for system tools. Unlike tool_names, distinguishes between tools that share a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    /// Filter conversations by IDs of tools that had successful calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids_successful: Option<Vec<String>>,
    /// Filter conversations by IDs of tools that had errored calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids_errored: Option<Vec<String>>,
    /// Also match tool calls that never ran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_invalid_tool_calls: Option<bool>,
    /// Filter conversations by detected main language (language code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_languages: Option<Vec<String>>,
    /// Filter conversations by agent phone number IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_ids: Option<Vec<String>>,
    /// Filter conversations by agent WhatsApp number IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_phone_number_ids: Option<Vec<String>>,
    /// How many conversations to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Whether to include transcript summaries in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_mode: Option<ListSummaryMode>,
    /// Full-text or fuzzy search over transcript messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ListConversationInitiationSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
    /// Restrict results to a single conversation product surface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_product_type: Option<ConversationProduct>,
    /// Filter conversations by branch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Filter conversations by version ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Filter conversations by parent conversation ID for subagent conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_conversation_id: Option<String>,
    /// Filter conversations by topic IDs assigned during topic discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_ids: Option<Vec<String>>,
    /// Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_statuses: Option<Vec<ConversationSummaryResponseModelStatus>>,
    /// Filter conversations by conversation tag IDs assigned via the conversation-tags endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// Filter conversations to only those that entered the given node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_entered_id: Option<String>,
    /// Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reasons: Option<Vec<String>>,
    /// Filter to conversations where a guardrail of any of these types triggered (metadata.triggered_guardrails.guardrail_type). Repeat param to match any of several.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_types: Option<Vec<GuardrailType>>,
    /// Filter to conversations where a custom guardrail with any of these names triggered (metadata.triggered_guardrails.guardrail_name). Only custom guardrails carry a name. Repeat param to match any of several.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_guardrail_names: Option<Vec<String>>,
    /// The direction to sort conversations by call start time. Defaults to descending (newest first).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort>,
}

impl ConversationsListQueryRequest {
    pub fn builder() -> ConversationsListQueryRequestBuilder {
        <ConversationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListQueryRequestBuilder {
    cursor: Option<String>,
    agent_id: Option<String>,
    visited_agent_ids: Option<Vec<String>>,
    visited_agent_branch_ids: Option<Vec<String>>,
    triggered_procedure_ids: Option<Vec<String>>,
    call_successful: Option<EvaluationResultFilter>,
    call_start_before_unix: Option<i64>,
    call_start_after_unix: Option<i64>,
    call_duration_min_secs: Option<i64>,
    call_duration_max_secs: Option<i64>,
    rating_max: Option<i64>,
    rating_min: Option<i64>,
    has_feedback_comment: Option<bool>,
    user_id: Option<String>,
    evaluation_params: Option<Vec<String>>,
    data_collection_params: Option<Vec<String>>,
    dynamic_variable_params: Option<Vec<String>>,
    data_collection_filter: Option<String>,
    dynamic_variable_filter: Option<String>,
    data_collection_ids: Option<Vec<String>>,
    evaluation_criteria_ids: Option<Vec<String>>,
    tool_names: Option<Vec<String>>,
    tool_names_successful: Option<Vec<String>>,
    tool_names_errored: Option<Vec<String>>,
    tool_ids: Option<Vec<String>>,
    tool_ids_successful: Option<Vec<String>>,
    tool_ids_errored: Option<Vec<String>>,
    include_invalid_tool_calls: Option<bool>,
    main_languages: Option<Vec<String>>,
    phone_number_ids: Option<Vec<String>>,
    whatsapp_phone_number_ids: Option<Vec<String>>,
    page_size: Option<i64>,
    summary_mode: Option<ListSummaryMode>,
    search: Option<String>,
    conversation_initiation_source: Option<ListConversationInitiationSource>,
    text_only: Option<bool>,
    conversation_product_type: Option<ConversationProduct>,
    branch_id: Option<String>,
    version_id: Option<String>,
    parent_conversation_id: Option<String>,
    topic_ids: Option<Vec<String>>,
    exclude_statuses: Option<Vec<ConversationSummaryResponseModelStatus>>,
    tag_ids: Option<Vec<String>>,
    workflow_node_entered_id: Option<String>,
    termination_reasons: Option<Vec<String>>,
    guardrail_types: Option<Vec<GuardrailType>>,
    custom_guardrail_names: Option<Vec<String>>,
    sort_direction: Option<BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort>,
}

impl ConversationsListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn visited_agent_ids(mut self, value: Vec<String>) -> Self {
        self.visited_agent_ids = Some(value);
        self
    }

    pub fn visited_agent_branch_ids(mut self, value: Vec<String>) -> Self {
        self.visited_agent_branch_ids = Some(value);
        self
    }

    pub fn triggered_procedure_ids(mut self, value: Vec<String>) -> Self {
        self.triggered_procedure_ids = Some(value);
        self
    }

    pub fn call_successful(mut self, value: EvaluationResultFilter) -> Self {
        self.call_successful = Some(value);
        self
    }

    pub fn call_start_before_unix(mut self, value: i64) -> Self {
        self.call_start_before_unix = Some(value);
        self
    }

    pub fn call_start_after_unix(mut self, value: i64) -> Self {
        self.call_start_after_unix = Some(value);
        self
    }

    pub fn call_duration_min_secs(mut self, value: i64) -> Self {
        self.call_duration_min_secs = Some(value);
        self
    }

    pub fn call_duration_max_secs(mut self, value: i64) -> Self {
        self.call_duration_max_secs = Some(value);
        self
    }

    pub fn rating_max(mut self, value: i64) -> Self {
        self.rating_max = Some(value);
        self
    }

    pub fn rating_min(mut self, value: i64) -> Self {
        self.rating_min = Some(value);
        self
    }

    pub fn has_feedback_comment(mut self, value: bool) -> Self {
        self.has_feedback_comment = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn evaluation_params(mut self, value: Vec<String>) -> Self {
        self.evaluation_params = Some(value);
        self
    }

    pub fn data_collection_params(mut self, value: Vec<String>) -> Self {
        self.data_collection_params = Some(value);
        self
    }

    pub fn dynamic_variable_params(mut self, value: Vec<String>) -> Self {
        self.dynamic_variable_params = Some(value);
        self
    }

    pub fn data_collection_filter(mut self, value: impl Into<String>) -> Self {
        self.data_collection_filter = Some(value.into());
        self
    }

    pub fn dynamic_variable_filter(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable_filter = Some(value.into());
        self
    }

    pub fn data_collection_ids(mut self, value: Vec<String>) -> Self {
        self.data_collection_ids = Some(value);
        self
    }

    pub fn evaluation_criteria_ids(mut self, value: Vec<String>) -> Self {
        self.evaluation_criteria_ids = Some(value);
        self
    }

    pub fn tool_names(mut self, value: Vec<String>) -> Self {
        self.tool_names = Some(value);
        self
    }

    pub fn tool_names_successful(mut self, value: Vec<String>) -> Self {
        self.tool_names_successful = Some(value);
        self
    }

    pub fn tool_names_errored(mut self, value: Vec<String>) -> Self {
        self.tool_names_errored = Some(value);
        self
    }

    pub fn tool_ids(mut self, value: Vec<String>) -> Self {
        self.tool_ids = Some(value);
        self
    }

    pub fn tool_ids_successful(mut self, value: Vec<String>) -> Self {
        self.tool_ids_successful = Some(value);
        self
    }

    pub fn tool_ids_errored(mut self, value: Vec<String>) -> Self {
        self.tool_ids_errored = Some(value);
        self
    }

    pub fn include_invalid_tool_calls(mut self, value: bool) -> Self {
        self.include_invalid_tool_calls = Some(value);
        self
    }

    pub fn main_languages(mut self, value: Vec<String>) -> Self {
        self.main_languages = Some(value);
        self
    }

    pub fn phone_number_ids(mut self, value: Vec<String>) -> Self {
        self.phone_number_ids = Some(value);
        self
    }

    pub fn whatsapp_phone_number_ids(mut self, value: Vec<String>) -> Self {
        self.whatsapp_phone_number_ids = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn summary_mode(mut self, value: ListSummaryMode) -> Self {
        self.summary_mode = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn conversation_initiation_source(
        mut self,
        value: ListConversationInitiationSource,
    ) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    pub fn conversation_product_type(mut self, value: ConversationProduct) -> Self {
        self.conversation_product_type = Some(value);
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn parent_conversation_id(mut self, value: impl Into<String>) -> Self {
        self.parent_conversation_id = Some(value.into());
        self
    }

    pub fn topic_ids(mut self, value: Vec<String>) -> Self {
        self.topic_ids = Some(value);
        self
    }

    pub fn exclude_statuses(mut self, value: Vec<ConversationSummaryResponseModelStatus>) -> Self {
        self.exclude_statuses = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn workflow_node_entered_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_entered_id = Some(value.into());
        self
    }

    pub fn termination_reasons(mut self, value: Vec<String>) -> Self {
        self.termination_reasons = Some(value);
        self
    }

    pub fn guardrail_types(mut self, value: Vec<GuardrailType>) -> Self {
        self.guardrail_types = Some(value);
        self
    }

    pub fn custom_guardrail_names(mut self, value: Vec<String>) -> Self {
        self.custom_guardrail_names = Some(value);
        self
    }

    pub fn sort_direction(
        mut self,
        value: BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort,
    ) -> Self {
        self.sort_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListQueryRequest`].
    pub fn build(self) -> Result<ConversationsListQueryRequest, BuildError> {
        Ok(ConversationsListQueryRequest {
            cursor: self.cursor,
            agent_id: self.agent_id,
            visited_agent_ids: self.visited_agent_ids,
            visited_agent_branch_ids: self.visited_agent_branch_ids,
            triggered_procedure_ids: self.triggered_procedure_ids,
            call_successful: self.call_successful,
            call_start_before_unix: self.call_start_before_unix,
            call_start_after_unix: self.call_start_after_unix,
            call_duration_min_secs: self.call_duration_min_secs,
            call_duration_max_secs: self.call_duration_max_secs,
            rating_max: self.rating_max,
            rating_min: self.rating_min,
            has_feedback_comment: self.has_feedback_comment,
            user_id: self.user_id,
            evaluation_params: self.evaluation_params,
            data_collection_params: self.data_collection_params,
            dynamic_variable_params: self.dynamic_variable_params,
            data_collection_filter: self.data_collection_filter,
            dynamic_variable_filter: self.dynamic_variable_filter,
            data_collection_ids: self.data_collection_ids,
            evaluation_criteria_ids: self.evaluation_criteria_ids,
            tool_names: self.tool_names,
            tool_names_successful: self.tool_names_successful,
            tool_names_errored: self.tool_names_errored,
            tool_ids: self.tool_ids,
            tool_ids_successful: self.tool_ids_successful,
            tool_ids_errored: self.tool_ids_errored,
            include_invalid_tool_calls: self.include_invalid_tool_calls,
            main_languages: self.main_languages,
            phone_number_ids: self.phone_number_ids,
            whatsapp_phone_number_ids: self.whatsapp_phone_number_ids,
            page_size: self.page_size,
            summary_mode: self.summary_mode,
            search: self.search,
            conversation_initiation_source: self.conversation_initiation_source,
            text_only: self.text_only,
            conversation_product_type: self.conversation_product_type,
            branch_id: self.branch_id,
            version_id: self.version_id,
            parent_conversation_id: self.parent_conversation_id,
            topic_ids: self.topic_ids,
            exclude_statuses: self.exclude_statuses,
            tag_ids: self.tag_ids,
            workflow_node_entered_id: self.workflow_node_entered_id,
            termination_reasons: self.termination_reasons,
            guardrail_types: self.guardrail_types,
            custom_guardrail_names: self.custom_guardrail_names,
            sort_direction: self.sort_direction,
        })
    }
}
