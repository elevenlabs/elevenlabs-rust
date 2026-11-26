pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WidgetTextContents {
    /// Call to action displayed inside the compact and full variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_label: Option<String>,
    /// Text and ARIA label for the start call button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_call: Option<String>,
    /// Text and ARIA label for the start chat button (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_chat: Option<String>,
    /// Text and ARIA label for the new call button. Displayed when the caller already finished at least one call in order ot start the next one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_call: Option<String>,
    /// Text and ARIA label for the button that resumes the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_call: Option<String>,
    /// Text and ARIA label for the end call button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call: Option<String>,
    /// Text and ARIA label for the end chat button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_chat: Option<String>,
    /// ARIA label for the mute microphone button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_microphone: Option<String>,
    /// ARIA label for the change language dropdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_language: Option<String>,
    /// ARIA label for the collapse button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse: Option<String>,
    /// ARIA label for the expand button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// ARIA label for the header button that expands the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_widget: Option<String>,
    /// ARIA label for the header button that collapses the widget back to its compact size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_widget: Option<String>,
    /// ARIA label for the button that hides a dismissible widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss: Option<String>,
    /// ARIA label for the launcher button that reopens a dismissed widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_chat: Option<String>,
    /// Text displayed when the user copies a value using the copy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copied: Option<String>,
    /// Text and ARIA label for the accept terms button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms: Option<String>,
    /// Text and ARIA label for the cancel terms button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss_terms: Option<String>,
    /// Title displayed in the confirmation dialog before ending a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_chat_confirmation_title: Option<String>,
    /// Description displayed in the confirmation dialog before ending a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_chat_confirmation_description: Option<String>,
    /// Title displayed in the confirmation dialog before ending a call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call_confirmation_title: Option<String>,
    /// Description displayed in the confirmation dialog before ending a call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call_confirmation_description: Option<String>,
    /// Text and ARIA label for the button that confirms ending a call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call_confirm: Option<String>,
    /// Text and ARIA label for the button that cancels ending a chat or call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_end_confirmation: Option<String>,
    /// Status displayed when the agent is listening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listening_status: Option<String>,
    /// Status displayed when the agent is speaking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaking_status: Option<String>,
    /// Status displayed when the agent is connecting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connecting_status: Option<String>,
    /// Status displayed when the agent is reconnecting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnecting_status: Option<String>,
    /// Status displayed when the agent is chatting (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatting_status: Option<String>,
    /// Status displayed while waiting in the queue for an available agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_waiting_status: Option<String>,
    /// Short status displayed while waiting for an available agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_waiting_status_short: Option<String>,
    /// ARIA label for the text message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_label: Option<String>,
    /// Placeholder text for the text message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_placeholder: Option<String>,
    /// Placeholder text for the text message input (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_placeholder_text_only: Option<String>,
    /// Placeholder text for the text message input when starting a new conversation (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_placeholder_new_conversation: Option<String>,
    /// Information message displayed when the user ends the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ended_conversation: Option<String>,
    /// Information message displayed when the agent ends the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ended_conversation: Option<String>,
    /// Text label used next to the conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Text label used when an error occurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_occurred: Option<String>,
    /// Error message displayed when the queue wait times out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_timed_out: Option<String>,
    /// Text and ARIA label used for the copy ID button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_id: Option<String>,
    /// Text displayed to prompt the user for feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_feedback: Option<String>,
    /// Text displayed to request additional feedback details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_follow_up_feedback: Option<String>,
    /// Text displayed to thank the user for providing feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thanks_for_feedback: Option<String>,
    /// Additional text displayed explaining the value of user feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thanks_for_feedback_details: Option<String>,
    /// Placeholder text for the follow-up feedback input field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_feedback_placeholder: Option<String>,
    /// Text and ARIA label for the submit button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<String>,
    /// Text and ARIA label for the go back button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_back: Option<String>,
    /// Text and ARIA label for the send message button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_message: Option<String>,
    /// Text and ARIA label for the switch to text mode button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mode: Option<String>,
    /// Text and ARIA label for the switch to voice mode button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_mode: Option<String>,
    /// Label for the voice option in the conversation mode toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_mode_label: Option<String>,
    /// Label for the text option in the conversation mode toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mode_label: Option<String>,
    /// Toast notification displayed when switching to text mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switched_to_text_mode: Option<String>,
    /// Toast notification displayed when switching to voice mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switched_to_voice_mode: Option<String>,
    /// Text and ARIA label for the copy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<String>,
    /// Text and ARIA label for the download button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    /// Text and ARIA label for the wrap toggle button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<String>,
    /// Status text displayed when the agent is processing a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_working: Option<String>,
    /// Status text displayed when the agent finishes processing a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_done: Option<String>,
    /// Status text displayed when the agent encounters an error during a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_error: Option<String>,
    /// Text and ARIA label for the attach file button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_file: Option<String>,
    /// ARIA label for the remove file button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_file: Option<String>,
    /// Error message displayed when a file fails to upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_upload_error: Option<String>,
    /// Error message displayed when an unsupported file type is selected. Followed by the list of accepted types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type_unsupported: Option<String>,
    /// Error message displayed when a file exceeds the maximum size limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_too_large: Option<String>,
    /// Error message displayed when the maximum number of files for a conversation is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_limit_reached: Option<String>,
    /// Status text displayed while the agent is typing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_indicator: Option<String>,
    /// Status text displayed while the agent is thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_status: Option<String>,
    /// Fallback message displayed when rich content cannot be rendered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_content_unavailable: Option<String>,
    /// ARIA label for the previous button in a rich content carousel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carousel_previous: Option<String>,
    /// ARIA label for the next button in a rich content carousel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carousel_next: Option<String>,
    /// Label for the button that scrolls to one new message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_message: Option<String>,
    /// Label for the button that scrolls to new messages. Keep the count placeholder from the default, which is replaced with the number of messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_messages: Option<String>,
}

impl WidgetTextContents {
    pub fn builder() -> WidgetTextContentsBuilder {
        <WidgetTextContentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetTextContentsBuilder {
    main_label: Option<String>,
    start_call: Option<String>,
    start_chat: Option<String>,
    new_call: Option<String>,
    resume_call: Option<String>,
    end_call: Option<String>,
    end_chat: Option<String>,
    mute_microphone: Option<String>,
    change_language: Option<String>,
    collapse: Option<String>,
    expand: Option<String>,
    expand_widget: Option<String>,
    collapse_widget: Option<String>,
    dismiss: Option<String>,
    open_chat: Option<String>,
    copied: Option<String>,
    accept_terms: Option<String>,
    dismiss_terms: Option<String>,
    end_chat_confirmation_title: Option<String>,
    end_chat_confirmation_description: Option<String>,
    end_call_confirmation_title: Option<String>,
    end_call_confirmation_description: Option<String>,
    end_call_confirm: Option<String>,
    cancel_end_confirmation: Option<String>,
    listening_status: Option<String>,
    speaking_status: Option<String>,
    connecting_status: Option<String>,
    reconnecting_status: Option<String>,
    chatting_status: Option<String>,
    queue_waiting_status: Option<String>,
    queue_waiting_status_short: Option<String>,
    input_label: Option<String>,
    input_placeholder: Option<String>,
    input_placeholder_text_only: Option<String>,
    input_placeholder_new_conversation: Option<String>,
    user_ended_conversation: Option<String>,
    agent_ended_conversation: Option<String>,
    conversation_id: Option<String>,
    error_occurred: Option<String>,
    queue_timed_out: Option<String>,
    copy_id: Option<String>,
    initiate_feedback: Option<String>,
    request_follow_up_feedback: Option<String>,
    thanks_for_feedback: Option<String>,
    thanks_for_feedback_details: Option<String>,
    follow_up_feedback_placeholder: Option<String>,
    submit: Option<String>,
    go_back: Option<String>,
    send_message: Option<String>,
    text_mode: Option<String>,
    voice_mode: Option<String>,
    voice_mode_label: Option<String>,
    text_mode_label: Option<String>,
    switched_to_text_mode: Option<String>,
    switched_to_voice_mode: Option<String>,
    copy: Option<String>,
    download: Option<String>,
    wrap: Option<String>,
    agent_working: Option<String>,
    agent_done: Option<String>,
    agent_error: Option<String>,
    attach_file: Option<String>,
    remove_file: Option<String>,
    file_upload_error: Option<String>,
    file_type_unsupported: Option<String>,
    file_too_large: Option<String>,
    file_limit_reached: Option<String>,
    typing_indicator: Option<String>,
    thinking_status: Option<String>,
    rich_content_unavailable: Option<String>,
    carousel_previous: Option<String>,
    carousel_next: Option<String>,
    new_message: Option<String>,
    new_messages: Option<String>,
}

impl WidgetTextContentsBuilder {
    pub fn main_label(mut self, value: impl Into<String>) -> Self {
        self.main_label = Some(value.into());
        self
    }

    pub fn start_call(mut self, value: impl Into<String>) -> Self {
        self.start_call = Some(value.into());
        self
    }

    pub fn start_chat(mut self, value: impl Into<String>) -> Self {
        self.start_chat = Some(value.into());
        self
    }

    pub fn new_call(mut self, value: impl Into<String>) -> Self {
        self.new_call = Some(value.into());
        self
    }

    pub fn resume_call(mut self, value: impl Into<String>) -> Self {
        self.resume_call = Some(value.into());
        self
    }

    pub fn end_call(mut self, value: impl Into<String>) -> Self {
        self.end_call = Some(value.into());
        self
    }

    pub fn end_chat(mut self, value: impl Into<String>) -> Self {
        self.end_chat = Some(value.into());
        self
    }

    pub fn mute_microphone(mut self, value: impl Into<String>) -> Self {
        self.mute_microphone = Some(value.into());
        self
    }

    pub fn change_language(mut self, value: impl Into<String>) -> Self {
        self.change_language = Some(value.into());
        self
    }

    pub fn collapse(mut self, value: impl Into<String>) -> Self {
        self.collapse = Some(value.into());
        self
    }

    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());
        self
    }

    pub fn expand_widget(mut self, value: impl Into<String>) -> Self {
        self.expand_widget = Some(value.into());
        self
    }

    pub fn collapse_widget(mut self, value: impl Into<String>) -> Self {
        self.collapse_widget = Some(value.into());
        self
    }

    pub fn dismiss(mut self, value: impl Into<String>) -> Self {
        self.dismiss = Some(value.into());
        self
    }

    pub fn open_chat(mut self, value: impl Into<String>) -> Self {
        self.open_chat = Some(value.into());
        self
    }

    pub fn copied(mut self, value: impl Into<String>) -> Self {
        self.copied = Some(value.into());
        self
    }

    pub fn accept_terms(mut self, value: impl Into<String>) -> Self {
        self.accept_terms = Some(value.into());
        self
    }

    pub fn dismiss_terms(mut self, value: impl Into<String>) -> Self {
        self.dismiss_terms = Some(value.into());
        self
    }

    pub fn end_chat_confirmation_title(mut self, value: impl Into<String>) -> Self {
        self.end_chat_confirmation_title = Some(value.into());
        self
    }

    pub fn end_chat_confirmation_description(mut self, value: impl Into<String>) -> Self {
        self.end_chat_confirmation_description = Some(value.into());
        self
    }

    pub fn end_call_confirmation_title(mut self, value: impl Into<String>) -> Self {
        self.end_call_confirmation_title = Some(value.into());
        self
    }

    pub fn end_call_confirmation_description(mut self, value: impl Into<String>) -> Self {
        self.end_call_confirmation_description = Some(value.into());
        self
    }

    pub fn end_call_confirm(mut self, value: impl Into<String>) -> Self {
        self.end_call_confirm = Some(value.into());
        self
    }

    pub fn cancel_end_confirmation(mut self, value: impl Into<String>) -> Self {
        self.cancel_end_confirmation = Some(value.into());
        self
    }

    pub fn listening_status(mut self, value: impl Into<String>) -> Self {
        self.listening_status = Some(value.into());
        self
    }

    pub fn speaking_status(mut self, value: impl Into<String>) -> Self {
        self.speaking_status = Some(value.into());
        self
    }

    pub fn connecting_status(mut self, value: impl Into<String>) -> Self {
        self.connecting_status = Some(value.into());
        self
    }

    pub fn reconnecting_status(mut self, value: impl Into<String>) -> Self {
        self.reconnecting_status = Some(value.into());
        self
    }

    pub fn chatting_status(mut self, value: impl Into<String>) -> Self {
        self.chatting_status = Some(value.into());
        self
    }

    pub fn queue_waiting_status(mut self, value: impl Into<String>) -> Self {
        self.queue_waiting_status = Some(value.into());
        self
    }

    pub fn queue_waiting_status_short(mut self, value: impl Into<String>) -> Self {
        self.queue_waiting_status_short = Some(value.into());
        self
    }

    pub fn input_label(mut self, value: impl Into<String>) -> Self {
        self.input_label = Some(value.into());
        self
    }

    pub fn input_placeholder(mut self, value: impl Into<String>) -> Self {
        self.input_placeholder = Some(value.into());
        self
    }

    pub fn input_placeholder_text_only(mut self, value: impl Into<String>) -> Self {
        self.input_placeholder_text_only = Some(value.into());
        self
    }

    pub fn input_placeholder_new_conversation(mut self, value: impl Into<String>) -> Self {
        self.input_placeholder_new_conversation = Some(value.into());
        self
    }

    pub fn user_ended_conversation(mut self, value: impl Into<String>) -> Self {
        self.user_ended_conversation = Some(value.into());
        self
    }

    pub fn agent_ended_conversation(mut self, value: impl Into<String>) -> Self {
        self.agent_ended_conversation = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn error_occurred(mut self, value: impl Into<String>) -> Self {
        self.error_occurred = Some(value.into());
        self
    }

    pub fn queue_timed_out(mut self, value: impl Into<String>) -> Self {
        self.queue_timed_out = Some(value.into());
        self
    }

    pub fn copy_id(mut self, value: impl Into<String>) -> Self {
        self.copy_id = Some(value.into());
        self
    }

    pub fn initiate_feedback(mut self, value: impl Into<String>) -> Self {
        self.initiate_feedback = Some(value.into());
        self
    }

    pub fn request_follow_up_feedback(mut self, value: impl Into<String>) -> Self {
        self.request_follow_up_feedback = Some(value.into());
        self
    }

    pub fn thanks_for_feedback(mut self, value: impl Into<String>) -> Self {
        self.thanks_for_feedback = Some(value.into());
        self
    }

    pub fn thanks_for_feedback_details(mut self, value: impl Into<String>) -> Self {
        self.thanks_for_feedback_details = Some(value.into());
        self
    }

    pub fn follow_up_feedback_placeholder(mut self, value: impl Into<String>) -> Self {
        self.follow_up_feedback_placeholder = Some(value.into());
        self
    }

    pub fn submit(mut self, value: impl Into<String>) -> Self {
        self.submit = Some(value.into());
        self
    }

    pub fn go_back(mut self, value: impl Into<String>) -> Self {
        self.go_back = Some(value.into());
        self
    }

    pub fn send_message(mut self, value: impl Into<String>) -> Self {
        self.send_message = Some(value.into());
        self
    }

    pub fn text_mode(mut self, value: impl Into<String>) -> Self {
        self.text_mode = Some(value.into());
        self
    }

    pub fn voice_mode(mut self, value: impl Into<String>) -> Self {
        self.voice_mode = Some(value.into());
        self
    }

    pub fn voice_mode_label(mut self, value: impl Into<String>) -> Self {
        self.voice_mode_label = Some(value.into());
        self
    }

    pub fn text_mode_label(mut self, value: impl Into<String>) -> Self {
        self.text_mode_label = Some(value.into());
        self
    }

    pub fn switched_to_text_mode(mut self, value: impl Into<String>) -> Self {
        self.switched_to_text_mode = Some(value.into());
        self
    }

    pub fn switched_to_voice_mode(mut self, value: impl Into<String>) -> Self {
        self.switched_to_voice_mode = Some(value.into());
        self
    }

    pub fn copy(mut self, value: impl Into<String>) -> Self {
        self.copy = Some(value.into());
        self
    }

    pub fn download(mut self, value: impl Into<String>) -> Self {
        self.download = Some(value.into());
        self
    }

    pub fn wrap(mut self, value: impl Into<String>) -> Self {
        self.wrap = Some(value.into());
        self
    }

    pub fn agent_working(mut self, value: impl Into<String>) -> Self {
        self.agent_working = Some(value.into());
        self
    }

    pub fn agent_done(mut self, value: impl Into<String>) -> Self {
        self.agent_done = Some(value.into());
        self
    }

    pub fn agent_error(mut self, value: impl Into<String>) -> Self {
        self.agent_error = Some(value.into());
        self
    }

    pub fn attach_file(mut self, value: impl Into<String>) -> Self {
        self.attach_file = Some(value.into());
        self
    }

    pub fn remove_file(mut self, value: impl Into<String>) -> Self {
        self.remove_file = Some(value.into());
        self
    }

    pub fn file_upload_error(mut self, value: impl Into<String>) -> Self {
        self.file_upload_error = Some(value.into());
        self
    }

    pub fn file_type_unsupported(mut self, value: impl Into<String>) -> Self {
        self.file_type_unsupported = Some(value.into());
        self
    }

    pub fn file_too_large(mut self, value: impl Into<String>) -> Self {
        self.file_too_large = Some(value.into());
        self
    }

    pub fn file_limit_reached(mut self, value: impl Into<String>) -> Self {
        self.file_limit_reached = Some(value.into());
        self
    }

    pub fn typing_indicator(mut self, value: impl Into<String>) -> Self {
        self.typing_indicator = Some(value.into());
        self
    }

    pub fn thinking_status(mut self, value: impl Into<String>) -> Self {
        self.thinking_status = Some(value.into());
        self
    }

    pub fn rich_content_unavailable(mut self, value: impl Into<String>) -> Self {
        self.rich_content_unavailable = Some(value.into());
        self
    }

    pub fn carousel_previous(mut self, value: impl Into<String>) -> Self {
        self.carousel_previous = Some(value.into());
        self
    }

    pub fn carousel_next(mut self, value: impl Into<String>) -> Self {
        self.carousel_next = Some(value.into());
        self
    }

    pub fn new_message(mut self, value: impl Into<String>) -> Self {
        self.new_message = Some(value.into());
        self
    }

    pub fn new_messages(mut self, value: impl Into<String>) -> Self {
        self.new_messages = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WidgetTextContents`].
    pub fn build(self) -> Result<WidgetTextContents, BuildError> {
        Ok(WidgetTextContents {
            main_label: self.main_label,
            start_call: self.start_call,
            start_chat: self.start_chat,
            new_call: self.new_call,
            resume_call: self.resume_call,
            end_call: self.end_call,
            end_chat: self.end_chat,
            mute_microphone: self.mute_microphone,
            change_language: self.change_language,
            collapse: self.collapse,
            expand: self.expand,
            expand_widget: self.expand_widget,
            collapse_widget: self.collapse_widget,
            dismiss: self.dismiss,
            open_chat: self.open_chat,
            copied: self.copied,
            accept_terms: self.accept_terms,
            dismiss_terms: self.dismiss_terms,
            end_chat_confirmation_title: self.end_chat_confirmation_title,
            end_chat_confirmation_description: self.end_chat_confirmation_description,
            end_call_confirmation_title: self.end_call_confirmation_title,
            end_call_confirmation_description: self.end_call_confirmation_description,
            end_call_confirm: self.end_call_confirm,
            cancel_end_confirmation: self.cancel_end_confirmation,
            listening_status: self.listening_status,
            speaking_status: self.speaking_status,
            connecting_status: self.connecting_status,
            reconnecting_status: self.reconnecting_status,
            chatting_status: self.chatting_status,
            queue_waiting_status: self.queue_waiting_status,
            queue_waiting_status_short: self.queue_waiting_status_short,
            input_label: self.input_label,
            input_placeholder: self.input_placeholder,
            input_placeholder_text_only: self.input_placeholder_text_only,
            input_placeholder_new_conversation: self.input_placeholder_new_conversation,
            user_ended_conversation: self.user_ended_conversation,
            agent_ended_conversation: self.agent_ended_conversation,
            conversation_id: self.conversation_id,
            error_occurred: self.error_occurred,
            queue_timed_out: self.queue_timed_out,
            copy_id: self.copy_id,
            initiate_feedback: self.initiate_feedback,
            request_follow_up_feedback: self.request_follow_up_feedback,
            thanks_for_feedback: self.thanks_for_feedback,
            thanks_for_feedback_details: self.thanks_for_feedback_details,
            follow_up_feedback_placeholder: self.follow_up_feedback_placeholder,
            submit: self.submit,
            go_back: self.go_back,
            send_message: self.send_message,
            text_mode: self.text_mode,
            voice_mode: self.voice_mode,
            voice_mode_label: self.voice_mode_label,
            text_mode_label: self.text_mode_label,
            switched_to_text_mode: self.switched_to_text_mode,
            switched_to_voice_mode: self.switched_to_voice_mode,
            copy: self.copy,
            download: self.download,
            wrap: self.wrap,
            agent_working: self.agent_working,
            agent_done: self.agent_done,
            agent_error: self.agent_error,
            attach_file: self.attach_file,
            remove_file: self.remove_file,
            file_upload_error: self.file_upload_error,
            file_type_unsupported: self.file_type_unsupported,
            file_too_large: self.file_too_large,
            file_limit_reached: self.file_limit_reached,
            typing_indicator: self.typing_indicator,
            thinking_status: self.thinking_status,
            rich_content_unavailable: self.rich_content_unavailable,
            carousel_previous: self.carousel_previous,
            carousel_next: self.carousel_next,
            new_message: self.new_message,
            new_messages: self.new_messages,
        })
    }
}
