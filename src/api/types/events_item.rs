pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventsItem {
    TicketMessageEventResponse(TicketMessageEventResponse),
    TicketCreatedEventResponse(TicketCreatedEventResponse),
    TicketDeletedEventResponse(TicketDeletedEventResponse),
    TicketAssigneeChangedEventResponse(TicketAssigneeChangedEventResponse),
    TicketSecondaryAssigneeChangedEventResponse(TicketSecondaryAssigneeChangedEventResponse),
    TicketSubscribersChangedEventResponse(TicketSubscribersChangedEventResponse),
    TicketSubjectChangedEventResponse(TicketSubjectChangedEventResponse),
    TicketPriorityChangedEventResponse(TicketPriorityChangedEventResponse),
    TicketStatusChangedEventResponse(TicketStatusChangedEventResponse),
    TicketDeadlineChangedEventResponse(TicketDeadlineChangedEventResponse),
    TicketTagIdsChangedEventResponse(TicketTagIdsChangedEventResponse),

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    __Unknown(serde_json::Value),
}

impl<'de> serde::Deserialize<'de> for EventsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("message") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<TicketMessageEventResponse>(value.clone())
            {
                return Ok(Self::TicketMessageEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("ticket_created") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<TicketCreatedEventResponse>(value.clone())
            {
                return Ok(Self::TicketCreatedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("ticket_deleted") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<TicketDeletedEventResponse>(value.clone())
            {
                return Ok(Self::TicketDeletedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("assignee_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketAssigneeChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketAssigneeChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str)
            == Some("secondary_assignee_changed")
        {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketSecondaryAssigneeChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketSecondaryAssigneeChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("subscribers_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketSubscribersChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketSubscribersChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("subject_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketSubjectChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketSubjectChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("priority_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketPriorityChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketPriorityChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("status_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketStatusChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketStatusChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("deadline_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketDeadlineChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketDeadlineChangedEventResponse(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("tag_ids_changed") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<TicketTagIdsChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketTagIdsChangedEventResponse(parsed));
            }
        }

        if !matched_literal {
            if value.get("type").is_some_and(serde_json::Value::is_string) {
                return Ok(Self::__Unknown(value));
            }

            if let Ok(parsed) = serde_json::from_value::<TicketMessageEventResponse>(value.clone())
            {
                return Ok(Self::TicketMessageEventResponse(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<TicketCreatedEventResponse>(value.clone())
            {
                return Ok(Self::TicketCreatedEventResponse(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<TicketDeletedEventResponse>(value.clone())
            {
                return Ok(Self::TicketDeletedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketAssigneeChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketAssigneeChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketSecondaryAssigneeChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketSecondaryAssigneeChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketSubscribersChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketSubscribersChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketSubjectChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketSubjectChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketPriorityChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketPriorityChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketStatusChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketStatusChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketDeadlineChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketDeadlineChangedEventResponse(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<TicketTagIdsChangedEventResponse>(value.clone())
            {
                return Ok(Self::TicketTagIdsChangedEventResponse(parsed));
            }

            return Ok(Self::__Unknown(value));
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum EventsItem",
        ))
    }
}

impl EventsItem {
    pub fn is_ticket_message_event_response(&self) -> bool {
        matches!(self, Self::TicketMessageEventResponse(_))
    }

    pub fn as_ticket_message_event_response(&self) -> Option<&TicketMessageEventResponse> {
        match self {
            Self::TicketMessageEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_message_event_response(self) -> Option<TicketMessageEventResponse> {
        match self {
            Self::TicketMessageEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_created_event_response(&self) -> bool {
        matches!(self, Self::TicketCreatedEventResponse(_))
    }

    pub fn as_ticket_created_event_response(&self) -> Option<&TicketCreatedEventResponse> {
        match self {
            Self::TicketCreatedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_created_event_response(self) -> Option<TicketCreatedEventResponse> {
        match self {
            Self::TicketCreatedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_deleted_event_response(&self) -> bool {
        matches!(self, Self::TicketDeletedEventResponse(_))
    }

    pub fn as_ticket_deleted_event_response(&self) -> Option<&TicketDeletedEventResponse> {
        match self {
            Self::TicketDeletedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_deleted_event_response(self) -> Option<TicketDeletedEventResponse> {
        match self {
            Self::TicketDeletedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_assignee_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketAssigneeChangedEventResponse(_))
    }

    pub fn as_ticket_assignee_changed_event_response(
        &self,
    ) -> Option<&TicketAssigneeChangedEventResponse> {
        match self {
            Self::TicketAssigneeChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_assignee_changed_event_response(
        self,
    ) -> Option<TicketAssigneeChangedEventResponse> {
        match self {
            Self::TicketAssigneeChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_secondary_assignee_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketSecondaryAssigneeChangedEventResponse(_))
    }

    pub fn as_ticket_secondary_assignee_changed_event_response(
        &self,
    ) -> Option<&TicketSecondaryAssigneeChangedEventResponse> {
        match self {
            Self::TicketSecondaryAssigneeChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_secondary_assignee_changed_event_response(
        self,
    ) -> Option<TicketSecondaryAssigneeChangedEventResponse> {
        match self {
            Self::TicketSecondaryAssigneeChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_subscribers_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketSubscribersChangedEventResponse(_))
    }

    pub fn as_ticket_subscribers_changed_event_response(
        &self,
    ) -> Option<&TicketSubscribersChangedEventResponse> {
        match self {
            Self::TicketSubscribersChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_subscribers_changed_event_response(
        self,
    ) -> Option<TicketSubscribersChangedEventResponse> {
        match self {
            Self::TicketSubscribersChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_subject_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketSubjectChangedEventResponse(_))
    }

    pub fn as_ticket_subject_changed_event_response(
        &self,
    ) -> Option<&TicketSubjectChangedEventResponse> {
        match self {
            Self::TicketSubjectChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_subject_changed_event_response(
        self,
    ) -> Option<TicketSubjectChangedEventResponse> {
        match self {
            Self::TicketSubjectChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_priority_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketPriorityChangedEventResponse(_))
    }

    pub fn as_ticket_priority_changed_event_response(
        &self,
    ) -> Option<&TicketPriorityChangedEventResponse> {
        match self {
            Self::TicketPriorityChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_priority_changed_event_response(
        self,
    ) -> Option<TicketPriorityChangedEventResponse> {
        match self {
            Self::TicketPriorityChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_status_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketStatusChangedEventResponse(_))
    }

    pub fn as_ticket_status_changed_event_response(
        &self,
    ) -> Option<&TicketStatusChangedEventResponse> {
        match self {
            Self::TicketStatusChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_status_changed_event_response(
        self,
    ) -> Option<TicketStatusChangedEventResponse> {
        match self {
            Self::TicketStatusChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_deadline_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketDeadlineChangedEventResponse(_))
    }

    pub fn as_ticket_deadline_changed_event_response(
        &self,
    ) -> Option<&TicketDeadlineChangedEventResponse> {
        match self {
            Self::TicketDeadlineChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_deadline_changed_event_response(
        self,
    ) -> Option<TicketDeadlineChangedEventResponse> {
        match self {
            Self::TicketDeadlineChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_ticket_tag_ids_changed_event_response(&self) -> bool {
        matches!(self, Self::TicketTagIdsChangedEventResponse(_))
    }

    pub fn as_ticket_tag_ids_changed_event_response(
        &self,
    ) -> Option<&TicketTagIdsChangedEventResponse> {
        match self {
            Self::TicketTagIdsChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ticket_tag_ids_changed_event_response(
        self,
    ) -> Option<TicketTagIdsChangedEventResponse> {
        match self {
            Self::TicketTagIdsChangedEventResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::__Unknown(_))
    }

    pub fn as_unknown(&self) -> Option<&serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_unknown(self) -> Option<serde_json::Value> {
        match self {
            Self::__Unknown(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for EventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TicketMessageEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketCreatedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketDeletedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketAssigneeChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketSecondaryAssigneeChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketSubscribersChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketSubjectChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketPriorityChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketStatusChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketDeadlineChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TicketTagIdsChangedEventResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::__Unknown(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
