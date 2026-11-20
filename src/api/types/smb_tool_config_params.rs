pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "smb_tool_type")]
#[non_exhaustive]
pub enum SmbToolConfigParams {
    #[serde(rename = "cancel_calendar_event")]
    CancelCalendarEvent {
        #[serde(flatten)]
        data: CancelCalendarEventParams,
    },

    #[serde(rename = "cancel_group_session_for_all")]
    CancelGroupSessionForAll {
        #[serde(flatten)]
        data: CancelGroupSessionForAllParams,
    },

    #[serde(rename = "cancel_group_session_registration")]
    CancelGroupSessionRegistration {
        #[serde(flatten)]
        data: CancelGroupSessionRegistrationParams,
    },

    #[serde(rename = "cancel_order")]
    CancelOrder {
        #[serde(flatten)]
        data: CancelOrderParams,
    },

    #[serde(rename = "check_service_availability")]
    CheckServiceAvailability {
        #[serde(flatten)]
        data: CheckServiceAvailabilityParams,
    },

    #[serde(rename = "create_agent_procedure")]
    CreateAgentProcedure {
        #[serde(flatten)]
        data: CreateAgentProcedureParams,
    },

    #[serde(rename = "create_agent_rule")]
    CreateAgentRule {
        #[serde(flatten)]
        data: CreateAgentRuleParams,
    },

    #[serde(rename = "create_asset")]
    CreateAsset {
        #[serde(flatten)]
        data: CreateAssetParams,
    },

    #[serde(rename = "create_client")]
    CreateClient {
        #[serde(flatten)]
        data: CreateClientParams,
    },

    #[serde(rename = "create_client_appointment")]
    CreateClientAppointment {
        #[serde(flatten)]
        data: CreateClientAppointmentParams,
    },

    #[serde(rename = "create_client_interaction")]
    CreateClientInteraction {
        #[serde(flatten)]
        data: CreateClientInteractionParams,
    },

    #[serde(rename = "create_holiday")]
    CreateHoliday {
        #[serde(flatten)]
        data: CreateHolidayParams,
    },

    #[serde(rename = "create_location")]
    CreateLocation {
        #[serde(flatten)]
        data: CreateLocationParams,
    },

    #[serde(rename = "create_order")]
    CreateOrder {
        #[serde(flatten)]
        data: CreateOrderParams,
    },

    #[serde(rename = "create_product")]
    CreateProduct {
        #[serde(flatten)]
        data: CreateProductParams,
    },

    #[serde(rename = "create_product_quote_request")]
    CreateProductQuoteRequest {
        #[serde(flatten)]
        data: CreateProductQuoteRequestParams,
    },

    #[serde(rename = "create_service")]
    CreateService {
        #[serde(flatten)]
        data: CreateServiceParams,
    },

    #[serde(rename = "create_service_quote_request")]
    CreateServiceQuoteRequest {
        #[serde(flatten)]
        data: CreateServiceQuoteRequestParams,
    },

    #[serde(rename = "create_staff")]
    CreateStaff {
        #[serde(flatten)]
        data: CreateStaffParams,
    },

    #[serde(rename = "create_transfer_rule")]
    CreateTransferRule {
        #[serde(flatten)]
        data: CreateTransferRuleParams,
    },

    #[serde(rename = "delete_agent_procedure")]
    DeleteAgentProcedure {
        #[serde(flatten)]
        data: DeleteAgentProcedureParams,
    },

    #[serde(rename = "delete_agent_rule")]
    DeleteAgentRule {
        #[serde(flatten)]
        data: DeleteAgentRuleParams,
    },

    #[serde(rename = "delete_asset")]
    DeleteAsset {
        #[serde(flatten)]
        data: DeleteAssetParams,
    },

    #[serde(rename = "delete_calendar_event")]
    DeleteCalendarEvent {
        #[serde(flatten)]
        data: DeleteCalendarEventParams,
    },

    #[serde(rename = "delete_client")]
    DeleteClient {
        #[serde(flatten)]
        data: DeleteClientParams,
    },

    #[serde(rename = "delete_client_interaction")]
    DeleteClientInteraction {
        #[serde(flatten)]
        data: DeleteClientInteractionParams,
    },

    #[serde(rename = "delete_group_session")]
    DeleteGroupSession {
        #[serde(flatten)]
        data: DeleteGroupSessionParams,
    },

    #[serde(rename = "delete_holiday")]
    DeleteHoliday {
        #[serde(flatten)]
        data: DeleteHolidayParams,
    },

    #[serde(rename = "delete_location")]
    DeleteLocation {
        #[serde(flatten)]
        data: DeleteLocationParams,
    },

    #[serde(rename = "delete_product")]
    DeleteProduct {
        #[serde(flatten)]
        data: DeleteProductParams,
    },

    #[serde(rename = "delete_service")]
    DeleteService {
        #[serde(flatten)]
        data: DeleteServiceParams,
    },

    #[serde(rename = "delete_staff")]
    DeleteStaff {
        #[serde(flatten)]
        data: DeleteStaffParams,
    },

    #[serde(rename = "delete_transfer_rule")]
    DeleteTransferRule {
        #[serde(flatten)]
        data: DeleteTransferRuleParams,
    },

    #[serde(rename = "get_agent_config")]
    GetAgentConfig {
        #[serde(flatten)]
        data: GetAgentConfigParams,
    },

    #[serde(rename = "get_analytics_summary")]
    GetAnalyticsSummary {
        #[serde(flatten)]
        data: GetAnalyticsSummaryParams,
    },

    #[serde(rename = "get_appointment_by_confirmation_number")]
    GetAppointmentByConfirmationNumber {
        #[serde(flatten)]
        data: GetAppointmentByConfirmationNumberParams,
    },

    #[serde(rename = "get_booking_page_settings")]
    GetBookingPageSettings {
        #[serde(flatten)]
        data: GetBookingPageSettingsParams,
    },

    #[serde(rename = "get_booking_slug_status")]
    GetBookingSlugStatus {
        #[serde(flatten)]
        data: GetBookingSlugStatusParams,
    },

    #[serde(rename = "get_client_appointments")]
    GetClientAppointments {
        #[serde(flatten)]
        data: GetClientAppointmentsParams,
    },

    #[serde(rename = "get_client_by_phone")]
    GetClientByPhone {
        #[serde(flatten)]
        data: GetClientByPhoneParams,
    },

    #[serde(rename = "get_client_orders")]
    GetClientOrders {
        #[serde(flatten)]
        data: GetClientOrdersParams,
    },

    #[serde(rename = "get_order_by_confirmation_number")]
    GetOrderByConfirmationNumber {
        #[serde(flatten)]
        data: GetOrderByConfirmationNumberParams,
    },

    #[serde(rename = "get_schedule")]
    GetSchedule {
        #[serde(flatten)]
        data: GetScheduleParams,
    },

    #[serde(rename = "get_workspace_plan")]
    GetWorkspacePlan {
        #[serde(flatten)]
        data: GetWorkspacePlanParams,
    },

    #[serde(rename = "leave_message")]
    LeaveMessage {
        #[serde(flatten)]
        data: LeaveMessageParams,
    },

    #[serde(rename = "list_agent_procedures")]
    ListAgentProcedures {
        #[serde(flatten)]
        data: ListAgentProceduresParams,
    },

    #[serde(rename = "list_agent_references")]
    ListAgentReferences {
        #[serde(flatten)]
        data: ListAgentReferencesParams,
    },

    #[serde(rename = "list_agent_rules")]
    ListAgentRules {
        #[serde(flatten)]
        data: ListAgentRulesParams,
    },

    #[serde(rename = "list_assets")]
    ListAssets {
        #[serde(flatten)]
        data: ListAssetsParams,
    },

    #[serde(rename = "list_calendar_events")]
    ListCalendarEvents {
        #[serde(flatten)]
        data: ListCalendarEventsParams,
    },

    #[serde(rename = "list_client_interactions")]
    ListClientInteractions {
        #[serde(flatten)]
        data: ListClientInteractionsParams,
    },

    #[serde(rename = "list_clients")]
    ListClients {
        #[serde(flatten)]
        data: ListClientsParams,
    },

    #[serde(rename = "list_customer_facing_agents")]
    ListCustomerFacingAgents {
        #[serde(flatten)]
        data: ListCustomerFacingAgentsParams,
    },

    #[serde(rename = "list_group_sessions")]
    ListGroupSessions {
        #[serde(flatten)]
        data: ListGroupSessionsParams,
    },

    #[serde(rename = "list_holidays")]
    ListHolidays {
        #[serde(flatten)]
        data: ListHolidaysParams,
    },

    #[serde(rename = "list_locations")]
    ListLocations {
        #[serde(flatten)]
        data: ListLocationsParams,
    },

    #[serde(rename = "list_products")]
    ListProducts {
        #[serde(flatten)]
        data: ListProductsParams,
    },

    #[serde(rename = "list_services")]
    ListServices {
        #[serde(flatten)]
        data: ListServicesParams,
    },

    #[serde(rename = "list_staff")]
    ListStaff {
        #[serde(flatten)]
        data: ListStaffParams,
    },

    #[serde(rename = "list_transfer_rules")]
    ListTransferRules {
        #[serde(flatten)]
        data: ListTransferRulesParams,
    },

    #[serde(rename = "opt_in_sms_reminder")]
    OptInSmsReminder {
        #[serde(flatten)]
        data: OptInSmsReminderParams,
    },

    #[serde(rename = "opt_out_sms_reminder")]
    OptOutSmsReminder {
        #[serde(flatten)]
        data: OptOutSmsReminderParams,
    },

    #[serde(rename = "owner_get_client_appointments")]
    OwnerGetClientAppointments {
        #[serde(flatten)]
        data: OwnerGetClientAppointmentsParams,
    },

    #[serde(rename = "owner_list_assets")]
    OwnerListAssets {
        #[serde(flatten)]
        data: OwnerListAssetsParams,
    },

    #[serde(rename = "owner_list_locations")]
    OwnerListLocations {
        #[serde(flatten)]
        data: OwnerListLocationsParams,
    },

    #[serde(rename = "owner_list_products")]
    OwnerListProducts {
        #[serde(flatten)]
        data: OwnerListProductsParams,
    },

    #[serde(rename = "owner_list_services")]
    OwnerListServices {
        #[serde(flatten)]
        data: OwnerListServicesParams,
    },

    #[serde(rename = "owner_list_staff")]
    OwnerListStaff {
        #[serde(flatten)]
        data: OwnerListStaffParams,
    },

    #[serde(rename = "register_for_group_session")]
    RegisterForGroupSession {
        #[serde(flatten)]
        data: RegisterForGroupSessionParams,
    },

    #[serde(rename = "report_knowledge_gap")]
    ReportKnowledgeGap {
        #[serde(flatten)]
        data: ReportKnowledgeGapParams,
    },

    #[serde(rename = "restore_calendar_event")]
    RestoreCalendarEvent {
        #[serde(flatten)]
        data: RestoreCalendarEventParams,
    },

    #[serde(rename = "schedule_group_session")]
    ScheduleGroupSession {
        #[serde(flatten)]
        data: ScheduleGroupSessionParams,
    },

    #[serde(rename = "search_clients")]
    SearchClients {
        #[serde(flatten)]
        data: SearchClientsParams,
    },

    #[serde(rename = "send_custom_email")]
    SendCustomEmail {
        #[serde(flatten)]
        data: SendCustomEmailParams,
    },

    #[serde(rename = "set_booking_slug")]
    SetBookingSlug {
        #[serde(flatten)]
        data: SetBookingSlugParams,
    },

    #[serde(rename = "smb_confirm_change")]
    SmbConfirmChange {
        #[serde(flatten)]
        data: ConfirmChangeParams,
    },

    #[serde(rename = "submit_business_info")]
    SubmitBusinessInfo {
        #[serde(flatten)]
        data: SubmitBusinessInfoParams,
    },

    #[serde(rename = "trigger_user_verification")]
    TriggerUserVerification {
        #[serde(flatten)]
        data: TriggerUserVerificationParams,
    },

    #[serde(rename = "update_agent_config")]
    UpdateAgentConfig {
        #[serde(flatten)]
        data: UpdateAgentConfigParams,
    },

    #[serde(rename = "update_agent_procedure")]
    UpdateAgentProcedure {
        #[serde(flatten)]
        data: UpdateAgentProcedureParams,
    },

    #[serde(rename = "update_agent_rule")]
    UpdateAgentRule {
        #[serde(flatten)]
        data: UpdateAgentRuleParams,
    },

    #[serde(rename = "update_asset")]
    UpdateAsset {
        #[serde(flatten)]
        data: UpdateAssetParams,
    },

    #[serde(rename = "update_booking_page_appearance")]
    UpdateBookingPageAppearance {
        #[serde(flatten)]
        data: UpdateBookingPageAppearanceParams,
    },

    #[serde(rename = "update_booking_page_settings")]
    UpdateBookingPageSettings {
        #[serde(flatten)]
        data: UpdateBookingPageSettingsParams,
    },

    #[serde(rename = "update_business_info")]
    UpdateBusinessInfo {
        #[serde(flatten)]
        data: UpdateBusinessInfoParams,
    },

    #[serde(rename = "update_calendar_event")]
    UpdateCalendarEvent {
        #[serde(flatten)]
        data: UpdateCalendarEventParams,
    },

    #[serde(rename = "update_client")]
    UpdateClient {
        #[serde(flatten)]
        data: UpdateClientParams,
    },

    #[serde(rename = "update_group_session_seats")]
    UpdateGroupSessionSeats {
        #[serde(flatten)]
        data: UpdateGroupSessionSeatsParams,
    },

    #[serde(rename = "update_holiday")]
    UpdateHoliday {
        #[serde(flatten)]
        data: UpdateHolidayParams,
    },

    #[serde(rename = "update_location")]
    UpdateLocation {
        #[serde(flatten)]
        data: UpdateLocationParams,
    },

    #[serde(rename = "update_order")]
    UpdateOrder {
        #[serde(flatten)]
        data: UpdateOrderParams,
    },

    #[serde(rename = "update_product")]
    UpdateProduct {
        #[serde(flatten)]
        data: UpdateProductParams,
    },

    #[serde(rename = "update_service")]
    UpdateService {
        #[serde(flatten)]
        data: UpdateServiceParams,
    },

    #[serde(rename = "update_staff")]
    UpdateStaff {
        #[serde(flatten)]
        data: UpdateStaffParams,
    },

    #[serde(rename = "update_transfer_rule")]
    UpdateTransferRule {
        #[serde(flatten)]
        data: UpdateTransferRuleParams,
    },

    #[serde(rename = "validate_user_verification_code")]
    ValidateUserVerificationCode {
        #[serde(flatten)]
        data: ValidateUserVerificationCodeParams,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SmbToolConfigParams {
    pub fn is_cancel_calendar_event(&self) -> bool {
        matches!(self, Self::CancelCalendarEvent { .. })
    }

    pub fn as_cancel_calendar_event(&self) -> Option<&CancelCalendarEventParams> {
        match self {
            Self::CancelCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_cancel_calendar_event(self) -> Option<CancelCalendarEventParams> {
        match self {
            Self::CancelCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_cancel_group_session_for_all(&self) -> bool {
        matches!(self, Self::CancelGroupSessionForAll { .. })
    }

    pub fn as_cancel_group_session_for_all(&self) -> Option<&CancelGroupSessionForAllParams> {
        match self {
            Self::CancelGroupSessionForAll { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_cancel_group_session_for_all(self) -> Option<CancelGroupSessionForAllParams> {
        match self {
            Self::CancelGroupSessionForAll { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_cancel_group_session_registration(&self) -> bool {
        matches!(self, Self::CancelGroupSessionRegistration { .. })
    }

    pub fn as_cancel_group_session_registration(
        &self,
    ) -> Option<&CancelGroupSessionRegistrationParams> {
        match self {
            Self::CancelGroupSessionRegistration { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_cancel_group_session_registration(
        self,
    ) -> Option<CancelGroupSessionRegistrationParams> {
        match self {
            Self::CancelGroupSessionRegistration { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_cancel_order(&self) -> bool {
        matches!(self, Self::CancelOrder { .. })
    }

    pub fn as_cancel_order(&self) -> Option<&CancelOrderParams> {
        match self {
            Self::CancelOrder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_cancel_order(self) -> Option<CancelOrderParams> {
        match self {
            Self::CancelOrder { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_check_service_availability(&self) -> bool {
        matches!(self, Self::CheckServiceAvailability { .. })
    }

    pub fn as_check_service_availability(&self) -> Option<&CheckServiceAvailabilityParams> {
        match self {
            Self::CheckServiceAvailability { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_check_service_availability(self) -> Option<CheckServiceAvailabilityParams> {
        match self {
            Self::CheckServiceAvailability { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_agent_procedure(&self) -> bool {
        matches!(self, Self::CreateAgentProcedure { .. })
    }

    pub fn as_create_agent_procedure(&self) -> Option<&CreateAgentProcedureParams> {
        match self {
            Self::CreateAgentProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_agent_procedure(self) -> Option<CreateAgentProcedureParams> {
        match self {
            Self::CreateAgentProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_agent_rule(&self) -> bool {
        matches!(self, Self::CreateAgentRule { .. })
    }

    pub fn as_create_agent_rule(&self) -> Option<&CreateAgentRuleParams> {
        match self {
            Self::CreateAgentRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_agent_rule(self) -> Option<CreateAgentRuleParams> {
        match self {
            Self::CreateAgentRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_asset(&self) -> bool {
        matches!(self, Self::CreateAsset { .. })
    }

    pub fn as_create_asset(&self) -> Option<&CreateAssetParams> {
        match self {
            Self::CreateAsset { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_asset(self) -> Option<CreateAssetParams> {
        match self {
            Self::CreateAsset { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_client(&self) -> bool {
        matches!(self, Self::CreateClient { .. })
    }

    pub fn as_create_client(&self) -> Option<&CreateClientParams> {
        match self {
            Self::CreateClient { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_client(self) -> Option<CreateClientParams> {
        match self {
            Self::CreateClient { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_client_appointment(&self) -> bool {
        matches!(self, Self::CreateClientAppointment { .. })
    }

    pub fn as_create_client_appointment(&self) -> Option<&CreateClientAppointmentParams> {
        match self {
            Self::CreateClientAppointment { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_client_appointment(self) -> Option<CreateClientAppointmentParams> {
        match self {
            Self::CreateClientAppointment { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_client_interaction(&self) -> bool {
        matches!(self, Self::CreateClientInteraction { .. })
    }

    pub fn as_create_client_interaction(&self) -> Option<&CreateClientInteractionParams> {
        match self {
            Self::CreateClientInteraction { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_client_interaction(self) -> Option<CreateClientInteractionParams> {
        match self {
            Self::CreateClientInteraction { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_holiday(&self) -> bool {
        matches!(self, Self::CreateHoliday { .. })
    }

    pub fn as_create_holiday(&self) -> Option<&CreateHolidayParams> {
        match self {
            Self::CreateHoliday { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_holiday(self) -> Option<CreateHolidayParams> {
        match self {
            Self::CreateHoliday { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_location(&self) -> bool {
        matches!(self, Self::CreateLocation { .. })
    }

    pub fn as_create_location(&self) -> Option<&CreateLocationParams> {
        match self {
            Self::CreateLocation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_location(self) -> Option<CreateLocationParams> {
        match self {
            Self::CreateLocation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_order(&self) -> bool {
        matches!(self, Self::CreateOrder { .. })
    }

    pub fn as_create_order(&self) -> Option<&CreateOrderParams> {
        match self {
            Self::CreateOrder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_order(self) -> Option<CreateOrderParams> {
        match self {
            Self::CreateOrder { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_product(&self) -> bool {
        matches!(self, Self::CreateProduct { .. })
    }

    pub fn as_create_product(&self) -> Option<&CreateProductParams> {
        match self {
            Self::CreateProduct { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_product(self) -> Option<CreateProductParams> {
        match self {
            Self::CreateProduct { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_product_quote_request(&self) -> bool {
        matches!(self, Self::CreateProductQuoteRequest { .. })
    }

    pub fn as_create_product_quote_request(&self) -> Option<&CreateProductQuoteRequestParams> {
        match self {
            Self::CreateProductQuoteRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_product_quote_request(self) -> Option<CreateProductQuoteRequestParams> {
        match self {
            Self::CreateProductQuoteRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_service(&self) -> bool {
        matches!(self, Self::CreateService { .. })
    }

    pub fn as_create_service(&self) -> Option<&CreateServiceParams> {
        match self {
            Self::CreateService { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_service(self) -> Option<CreateServiceParams> {
        match self {
            Self::CreateService { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_service_quote_request(&self) -> bool {
        matches!(self, Self::CreateServiceQuoteRequest { .. })
    }

    pub fn as_create_service_quote_request(&self) -> Option<&CreateServiceQuoteRequestParams> {
        match self {
            Self::CreateServiceQuoteRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_service_quote_request(self) -> Option<CreateServiceQuoteRequestParams> {
        match self {
            Self::CreateServiceQuoteRequest { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_staff(&self) -> bool {
        matches!(self, Self::CreateStaff { .. })
    }

    pub fn as_create_staff(&self) -> Option<&CreateStaffParams> {
        match self {
            Self::CreateStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_staff(self) -> Option<CreateStaffParams> {
        match self {
            Self::CreateStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_create_transfer_rule(&self) -> bool {
        matches!(self, Self::CreateTransferRule { .. })
    }

    pub fn as_create_transfer_rule(&self) -> Option<&CreateTransferRuleParams> {
        match self {
            Self::CreateTransferRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_create_transfer_rule(self) -> Option<CreateTransferRuleParams> {
        match self {
            Self::CreateTransferRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_agent_procedure(&self) -> bool {
        matches!(self, Self::DeleteAgentProcedure { .. })
    }

    pub fn as_delete_agent_procedure(&self) -> Option<&DeleteAgentProcedureParams> {
        match self {
            Self::DeleteAgentProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_agent_procedure(self) -> Option<DeleteAgentProcedureParams> {
        match self {
            Self::DeleteAgentProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_agent_rule(&self) -> bool {
        matches!(self, Self::DeleteAgentRule { .. })
    }

    pub fn as_delete_agent_rule(&self) -> Option<&DeleteAgentRuleParams> {
        match self {
            Self::DeleteAgentRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_agent_rule(self) -> Option<DeleteAgentRuleParams> {
        match self {
            Self::DeleteAgentRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_asset(&self) -> bool {
        matches!(self, Self::DeleteAsset { .. })
    }

    pub fn as_delete_asset(&self) -> Option<&DeleteAssetParams> {
        match self {
            Self::DeleteAsset { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_asset(self) -> Option<DeleteAssetParams> {
        match self {
            Self::DeleteAsset { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_calendar_event(&self) -> bool {
        matches!(self, Self::DeleteCalendarEvent { .. })
    }

    pub fn as_delete_calendar_event(&self) -> Option<&DeleteCalendarEventParams> {
        match self {
            Self::DeleteCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_calendar_event(self) -> Option<DeleteCalendarEventParams> {
        match self {
            Self::DeleteCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_client(&self) -> bool {
        matches!(self, Self::DeleteClient { .. })
    }

    pub fn as_delete_client(&self) -> Option<&DeleteClientParams> {
        match self {
            Self::DeleteClient { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_client(self) -> Option<DeleteClientParams> {
        match self {
            Self::DeleteClient { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_client_interaction(&self) -> bool {
        matches!(self, Self::DeleteClientInteraction { .. })
    }

    pub fn as_delete_client_interaction(&self) -> Option<&DeleteClientInteractionParams> {
        match self {
            Self::DeleteClientInteraction { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_client_interaction(self) -> Option<DeleteClientInteractionParams> {
        match self {
            Self::DeleteClientInteraction { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_group_session(&self) -> bool {
        matches!(self, Self::DeleteGroupSession { .. })
    }

    pub fn as_delete_group_session(&self) -> Option<&DeleteGroupSessionParams> {
        match self {
            Self::DeleteGroupSession { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_group_session(self) -> Option<DeleteGroupSessionParams> {
        match self {
            Self::DeleteGroupSession { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_holiday(&self) -> bool {
        matches!(self, Self::DeleteHoliday { .. })
    }

    pub fn as_delete_holiday(&self) -> Option<&DeleteHolidayParams> {
        match self {
            Self::DeleteHoliday { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_holiday(self) -> Option<DeleteHolidayParams> {
        match self {
            Self::DeleteHoliday { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_location(&self) -> bool {
        matches!(self, Self::DeleteLocation { .. })
    }

    pub fn as_delete_location(&self) -> Option<&DeleteLocationParams> {
        match self {
            Self::DeleteLocation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_location(self) -> Option<DeleteLocationParams> {
        match self {
            Self::DeleteLocation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_product(&self) -> bool {
        matches!(self, Self::DeleteProduct { .. })
    }

    pub fn as_delete_product(&self) -> Option<&DeleteProductParams> {
        match self {
            Self::DeleteProduct { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_product(self) -> Option<DeleteProductParams> {
        match self {
            Self::DeleteProduct { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_service(&self) -> bool {
        matches!(self, Self::DeleteService { .. })
    }

    pub fn as_delete_service(&self) -> Option<&DeleteServiceParams> {
        match self {
            Self::DeleteService { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_service(self) -> Option<DeleteServiceParams> {
        match self {
            Self::DeleteService { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_staff(&self) -> bool {
        matches!(self, Self::DeleteStaff { .. })
    }

    pub fn as_delete_staff(&self) -> Option<&DeleteStaffParams> {
        match self {
            Self::DeleteStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_staff(self) -> Option<DeleteStaffParams> {
        match self {
            Self::DeleteStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_delete_transfer_rule(&self) -> bool {
        matches!(self, Self::DeleteTransferRule { .. })
    }

    pub fn as_delete_transfer_rule(&self) -> Option<&DeleteTransferRuleParams> {
        match self {
            Self::DeleteTransferRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_delete_transfer_rule(self) -> Option<DeleteTransferRuleParams> {
        match self {
            Self::DeleteTransferRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_agent_config(&self) -> bool {
        matches!(self, Self::GetAgentConfig { .. })
    }

    pub fn as_get_agent_config(&self) -> Option<&GetAgentConfigParams> {
        match self {
            Self::GetAgentConfig { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_agent_config(self) -> Option<GetAgentConfigParams> {
        match self {
            Self::GetAgentConfig { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_analytics_summary(&self) -> bool {
        matches!(self, Self::GetAnalyticsSummary { .. })
    }

    pub fn as_get_analytics_summary(&self) -> Option<&GetAnalyticsSummaryParams> {
        match self {
            Self::GetAnalyticsSummary { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_analytics_summary(self) -> Option<GetAnalyticsSummaryParams> {
        match self {
            Self::GetAnalyticsSummary { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_appointment_by_confirmation_number(&self) -> bool {
        matches!(self, Self::GetAppointmentByConfirmationNumber { .. })
    }

    pub fn as_get_appointment_by_confirmation_number(
        &self,
    ) -> Option<&GetAppointmentByConfirmationNumberParams> {
        match self {
            Self::GetAppointmentByConfirmationNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_appointment_by_confirmation_number(
        self,
    ) -> Option<GetAppointmentByConfirmationNumberParams> {
        match self {
            Self::GetAppointmentByConfirmationNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_booking_page_settings(&self) -> bool {
        matches!(self, Self::GetBookingPageSettings { .. })
    }

    pub fn as_get_booking_page_settings(&self) -> Option<&GetBookingPageSettingsParams> {
        match self {
            Self::GetBookingPageSettings { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_booking_page_settings(self) -> Option<GetBookingPageSettingsParams> {
        match self {
            Self::GetBookingPageSettings { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_booking_slug_status(&self) -> bool {
        matches!(self, Self::GetBookingSlugStatus { .. })
    }

    pub fn as_get_booking_slug_status(&self) -> Option<&GetBookingSlugStatusParams> {
        match self {
            Self::GetBookingSlugStatus { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_booking_slug_status(self) -> Option<GetBookingSlugStatusParams> {
        match self {
            Self::GetBookingSlugStatus { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_client_appointments(&self) -> bool {
        matches!(self, Self::GetClientAppointments { .. })
    }

    pub fn as_get_client_appointments(&self) -> Option<&GetClientAppointmentsParams> {
        match self {
            Self::GetClientAppointments { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_client_appointments(self) -> Option<GetClientAppointmentsParams> {
        match self {
            Self::GetClientAppointments { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_client_by_phone(&self) -> bool {
        matches!(self, Self::GetClientByPhone { .. })
    }

    pub fn as_get_client_by_phone(&self) -> Option<&GetClientByPhoneParams> {
        match self {
            Self::GetClientByPhone { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_client_by_phone(self) -> Option<GetClientByPhoneParams> {
        match self {
            Self::GetClientByPhone { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_client_orders(&self) -> bool {
        matches!(self, Self::GetClientOrders { .. })
    }

    pub fn as_get_client_orders(&self) -> Option<&GetClientOrdersParams> {
        match self {
            Self::GetClientOrders { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_client_orders(self) -> Option<GetClientOrdersParams> {
        match self {
            Self::GetClientOrders { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_order_by_confirmation_number(&self) -> bool {
        matches!(self, Self::GetOrderByConfirmationNumber { .. })
    }

    pub fn as_get_order_by_confirmation_number(
        &self,
    ) -> Option<&GetOrderByConfirmationNumberParams> {
        match self {
            Self::GetOrderByConfirmationNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_order_by_confirmation_number(
        self,
    ) -> Option<GetOrderByConfirmationNumberParams> {
        match self {
            Self::GetOrderByConfirmationNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_schedule(&self) -> bool {
        matches!(self, Self::GetSchedule { .. })
    }

    pub fn as_get_schedule(&self) -> Option<&GetScheduleParams> {
        match self {
            Self::GetSchedule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_schedule(self) -> Option<GetScheduleParams> {
        match self {
            Self::GetSchedule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_get_workspace_plan(&self) -> bool {
        matches!(self, Self::GetWorkspacePlan { .. })
    }

    pub fn as_get_workspace_plan(&self) -> Option<&GetWorkspacePlanParams> {
        match self {
            Self::GetWorkspacePlan { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_get_workspace_plan(self) -> Option<GetWorkspacePlanParams> {
        match self {
            Self::GetWorkspacePlan { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_leave_message(&self) -> bool {
        matches!(self, Self::LeaveMessage { .. })
    }

    pub fn as_leave_message(&self) -> Option<&LeaveMessageParams> {
        match self {
            Self::LeaveMessage { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_leave_message(self) -> Option<LeaveMessageParams> {
        match self {
            Self::LeaveMessage { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_agent_procedures(&self) -> bool {
        matches!(self, Self::ListAgentProcedures { .. })
    }

    pub fn as_list_agent_procedures(&self) -> Option<&ListAgentProceduresParams> {
        match self {
            Self::ListAgentProcedures { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_agent_procedures(self) -> Option<ListAgentProceduresParams> {
        match self {
            Self::ListAgentProcedures { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_agent_references(&self) -> bool {
        matches!(self, Self::ListAgentReferences { .. })
    }

    pub fn as_list_agent_references(&self) -> Option<&ListAgentReferencesParams> {
        match self {
            Self::ListAgentReferences { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_agent_references(self) -> Option<ListAgentReferencesParams> {
        match self {
            Self::ListAgentReferences { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_agent_rules(&self) -> bool {
        matches!(self, Self::ListAgentRules { .. })
    }

    pub fn as_list_agent_rules(&self) -> Option<&ListAgentRulesParams> {
        match self {
            Self::ListAgentRules { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_agent_rules(self) -> Option<ListAgentRulesParams> {
        match self {
            Self::ListAgentRules { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_assets(&self) -> bool {
        matches!(self, Self::ListAssets { .. })
    }

    pub fn as_list_assets(&self) -> Option<&ListAssetsParams> {
        match self {
            Self::ListAssets { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_assets(self) -> Option<ListAssetsParams> {
        match self {
            Self::ListAssets { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_calendar_events(&self) -> bool {
        matches!(self, Self::ListCalendarEvents { .. })
    }

    pub fn as_list_calendar_events(&self) -> Option<&ListCalendarEventsParams> {
        match self {
            Self::ListCalendarEvents { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_calendar_events(self) -> Option<ListCalendarEventsParams> {
        match self {
            Self::ListCalendarEvents { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_client_interactions(&self) -> bool {
        matches!(self, Self::ListClientInteractions { .. })
    }

    pub fn as_list_client_interactions(&self) -> Option<&ListClientInteractionsParams> {
        match self {
            Self::ListClientInteractions { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_client_interactions(self) -> Option<ListClientInteractionsParams> {
        match self {
            Self::ListClientInteractions { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_clients(&self) -> bool {
        matches!(self, Self::ListClients { .. })
    }

    pub fn as_list_clients(&self) -> Option<&ListClientsParams> {
        match self {
            Self::ListClients { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_clients(self) -> Option<ListClientsParams> {
        match self {
            Self::ListClients { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_customer_facing_agents(&self) -> bool {
        matches!(self, Self::ListCustomerFacingAgents { .. })
    }

    pub fn as_list_customer_facing_agents(&self) -> Option<&ListCustomerFacingAgentsParams> {
        match self {
            Self::ListCustomerFacingAgents { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_customer_facing_agents(self) -> Option<ListCustomerFacingAgentsParams> {
        match self {
            Self::ListCustomerFacingAgents { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_group_sessions(&self) -> bool {
        matches!(self, Self::ListGroupSessions { .. })
    }

    pub fn as_list_group_sessions(&self) -> Option<&ListGroupSessionsParams> {
        match self {
            Self::ListGroupSessions { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_group_sessions(self) -> Option<ListGroupSessionsParams> {
        match self {
            Self::ListGroupSessions { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_holidays(&self) -> bool {
        matches!(self, Self::ListHolidays { .. })
    }

    pub fn as_list_holidays(&self) -> Option<&ListHolidaysParams> {
        match self {
            Self::ListHolidays { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_holidays(self) -> Option<ListHolidaysParams> {
        match self {
            Self::ListHolidays { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_locations(&self) -> bool {
        matches!(self, Self::ListLocations { .. })
    }

    pub fn as_list_locations(&self) -> Option<&ListLocationsParams> {
        match self {
            Self::ListLocations { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_locations(self) -> Option<ListLocationsParams> {
        match self {
            Self::ListLocations { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_products(&self) -> bool {
        matches!(self, Self::ListProducts { .. })
    }

    pub fn as_list_products(&self) -> Option<&ListProductsParams> {
        match self {
            Self::ListProducts { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_products(self) -> Option<ListProductsParams> {
        match self {
            Self::ListProducts { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_services(&self) -> bool {
        matches!(self, Self::ListServices { .. })
    }

    pub fn as_list_services(&self) -> Option<&ListServicesParams> {
        match self {
            Self::ListServices { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_services(self) -> Option<ListServicesParams> {
        match self {
            Self::ListServices { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_staff(&self) -> bool {
        matches!(self, Self::ListStaff { .. })
    }

    pub fn as_list_staff(&self) -> Option<&ListStaffParams> {
        match self {
            Self::ListStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_staff(self) -> Option<ListStaffParams> {
        match self {
            Self::ListStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_list_transfer_rules(&self) -> bool {
        matches!(self, Self::ListTransferRules { .. })
    }

    pub fn as_list_transfer_rules(&self) -> Option<&ListTransferRulesParams> {
        match self {
            Self::ListTransferRules { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_list_transfer_rules(self) -> Option<ListTransferRulesParams> {
        match self {
            Self::ListTransferRules { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_opt_in_sms_reminder(&self) -> bool {
        matches!(self, Self::OptInSmsReminder { .. })
    }

    pub fn as_opt_in_sms_reminder(&self) -> Option<&OptInSmsReminderParams> {
        match self {
            Self::OptInSmsReminder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_opt_in_sms_reminder(self) -> Option<OptInSmsReminderParams> {
        match self {
            Self::OptInSmsReminder { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_opt_out_sms_reminder(&self) -> bool {
        matches!(self, Self::OptOutSmsReminder { .. })
    }

    pub fn as_opt_out_sms_reminder(&self) -> Option<&OptOutSmsReminderParams> {
        match self {
            Self::OptOutSmsReminder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_opt_out_sms_reminder(self) -> Option<OptOutSmsReminderParams> {
        match self {
            Self::OptOutSmsReminder { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_owner_get_client_appointments(&self) -> bool {
        matches!(self, Self::OwnerGetClientAppointments { .. })
    }

    pub fn as_owner_get_client_appointments(&self) -> Option<&OwnerGetClientAppointmentsParams> {
        match self {
            Self::OwnerGetClientAppointments { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_owner_get_client_appointments(self) -> Option<OwnerGetClientAppointmentsParams> {
        match self {
            Self::OwnerGetClientAppointments { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_owner_list_assets(&self) -> bool {
        matches!(self, Self::OwnerListAssets { .. })
    }

    pub fn as_owner_list_assets(&self) -> Option<&OwnerListAssetsParams> {
        match self {
            Self::OwnerListAssets { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_owner_list_assets(self) -> Option<OwnerListAssetsParams> {
        match self {
            Self::OwnerListAssets { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_owner_list_locations(&self) -> bool {
        matches!(self, Self::OwnerListLocations { .. })
    }

    pub fn as_owner_list_locations(&self) -> Option<&OwnerListLocationsParams> {
        match self {
            Self::OwnerListLocations { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_owner_list_locations(self) -> Option<OwnerListLocationsParams> {
        match self {
            Self::OwnerListLocations { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_owner_list_products(&self) -> bool {
        matches!(self, Self::OwnerListProducts { .. })
    }

    pub fn as_owner_list_products(&self) -> Option<&OwnerListProductsParams> {
        match self {
            Self::OwnerListProducts { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_owner_list_products(self) -> Option<OwnerListProductsParams> {
        match self {
            Self::OwnerListProducts { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_owner_list_services(&self) -> bool {
        matches!(self, Self::OwnerListServices { .. })
    }

    pub fn as_owner_list_services(&self) -> Option<&OwnerListServicesParams> {
        match self {
            Self::OwnerListServices { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_owner_list_services(self) -> Option<OwnerListServicesParams> {
        match self {
            Self::OwnerListServices { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_owner_list_staff(&self) -> bool {
        matches!(self, Self::OwnerListStaff { .. })
    }

    pub fn as_owner_list_staff(&self) -> Option<&OwnerListStaffParams> {
        match self {
            Self::OwnerListStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_owner_list_staff(self) -> Option<OwnerListStaffParams> {
        match self {
            Self::OwnerListStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_register_for_group_session(&self) -> bool {
        matches!(self, Self::RegisterForGroupSession { .. })
    }

    pub fn as_register_for_group_session(&self) -> Option<&RegisterForGroupSessionParams> {
        match self {
            Self::RegisterForGroupSession { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_register_for_group_session(self) -> Option<RegisterForGroupSessionParams> {
        match self {
            Self::RegisterForGroupSession { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_report_knowledge_gap(&self) -> bool {
        matches!(self, Self::ReportKnowledgeGap { .. })
    }

    pub fn as_report_knowledge_gap(&self) -> Option<&ReportKnowledgeGapParams> {
        match self {
            Self::ReportKnowledgeGap { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_report_knowledge_gap(self) -> Option<ReportKnowledgeGapParams> {
        match self {
            Self::ReportKnowledgeGap { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_restore_calendar_event(&self) -> bool {
        matches!(self, Self::RestoreCalendarEvent { .. })
    }

    pub fn as_restore_calendar_event(&self) -> Option<&RestoreCalendarEventParams> {
        match self {
            Self::RestoreCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_restore_calendar_event(self) -> Option<RestoreCalendarEventParams> {
        match self {
            Self::RestoreCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_schedule_group_session(&self) -> bool {
        matches!(self, Self::ScheduleGroupSession { .. })
    }

    pub fn as_schedule_group_session(&self) -> Option<&ScheduleGroupSessionParams> {
        match self {
            Self::ScheduleGroupSession { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_schedule_group_session(self) -> Option<ScheduleGroupSessionParams> {
        match self {
            Self::ScheduleGroupSession { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_search_clients(&self) -> bool {
        matches!(self, Self::SearchClients { .. })
    }

    pub fn as_search_clients(&self) -> Option<&SearchClientsParams> {
        match self {
            Self::SearchClients { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_search_clients(self) -> Option<SearchClientsParams> {
        match self {
            Self::SearchClients { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_send_custom_email(&self) -> bool {
        matches!(self, Self::SendCustomEmail { .. })
    }

    pub fn as_send_custom_email(&self) -> Option<&SendCustomEmailParams> {
        match self {
            Self::SendCustomEmail { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_send_custom_email(self) -> Option<SendCustomEmailParams> {
        match self {
            Self::SendCustomEmail { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_set_booking_slug(&self) -> bool {
        matches!(self, Self::SetBookingSlug { .. })
    }

    pub fn as_set_booking_slug(&self) -> Option<&SetBookingSlugParams> {
        match self {
            Self::SetBookingSlug { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_set_booking_slug(self) -> Option<SetBookingSlugParams> {
        match self {
            Self::SetBookingSlug { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_smb_confirm_change(&self) -> bool {
        matches!(self, Self::SmbConfirmChange { .. })
    }

    pub fn as_smb_confirm_change(&self) -> Option<&ConfirmChangeParams> {
        match self {
            Self::SmbConfirmChange { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_smb_confirm_change(self) -> Option<ConfirmChangeParams> {
        match self {
            Self::SmbConfirmChange { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_submit_business_info(&self) -> bool {
        matches!(self, Self::SubmitBusinessInfo { .. })
    }

    pub fn as_submit_business_info(&self) -> Option<&SubmitBusinessInfoParams> {
        match self {
            Self::SubmitBusinessInfo { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_submit_business_info(self) -> Option<SubmitBusinessInfoParams> {
        match self {
            Self::SubmitBusinessInfo { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_trigger_user_verification(&self) -> bool {
        matches!(self, Self::TriggerUserVerification { .. })
    }

    pub fn as_trigger_user_verification(&self) -> Option<&TriggerUserVerificationParams> {
        match self {
            Self::TriggerUserVerification { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_trigger_user_verification(self) -> Option<TriggerUserVerificationParams> {
        match self {
            Self::TriggerUserVerification { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_agent_config(&self) -> bool {
        matches!(self, Self::UpdateAgentConfig { .. })
    }

    pub fn as_update_agent_config(&self) -> Option<&UpdateAgentConfigParams> {
        match self {
            Self::UpdateAgentConfig { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_agent_config(self) -> Option<UpdateAgentConfigParams> {
        match self {
            Self::UpdateAgentConfig { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_agent_procedure(&self) -> bool {
        matches!(self, Self::UpdateAgentProcedure { .. })
    }

    pub fn as_update_agent_procedure(&self) -> Option<&UpdateAgentProcedureParams> {
        match self {
            Self::UpdateAgentProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_agent_procedure(self) -> Option<UpdateAgentProcedureParams> {
        match self {
            Self::UpdateAgentProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_agent_rule(&self) -> bool {
        matches!(self, Self::UpdateAgentRule { .. })
    }

    pub fn as_update_agent_rule(&self) -> Option<&UpdateAgentRuleParams> {
        match self {
            Self::UpdateAgentRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_agent_rule(self) -> Option<UpdateAgentRuleParams> {
        match self {
            Self::UpdateAgentRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_asset(&self) -> bool {
        matches!(self, Self::UpdateAsset { .. })
    }

    pub fn as_update_asset(&self) -> Option<&UpdateAssetParams> {
        match self {
            Self::UpdateAsset { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_asset(self) -> Option<UpdateAssetParams> {
        match self {
            Self::UpdateAsset { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_booking_page_appearance(&self) -> bool {
        matches!(self, Self::UpdateBookingPageAppearance { .. })
    }

    pub fn as_update_booking_page_appearance(&self) -> Option<&UpdateBookingPageAppearanceParams> {
        match self {
            Self::UpdateBookingPageAppearance { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_booking_page_appearance(self) -> Option<UpdateBookingPageAppearanceParams> {
        match self {
            Self::UpdateBookingPageAppearance { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_booking_page_settings(&self) -> bool {
        matches!(self, Self::UpdateBookingPageSettings { .. })
    }

    pub fn as_update_booking_page_settings(&self) -> Option<&UpdateBookingPageSettingsParams> {
        match self {
            Self::UpdateBookingPageSettings { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_booking_page_settings(self) -> Option<UpdateBookingPageSettingsParams> {
        match self {
            Self::UpdateBookingPageSettings { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_business_info(&self) -> bool {
        matches!(self, Self::UpdateBusinessInfo { .. })
    }

    pub fn as_update_business_info(&self) -> Option<&UpdateBusinessInfoParams> {
        match self {
            Self::UpdateBusinessInfo { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_business_info(self) -> Option<UpdateBusinessInfoParams> {
        match self {
            Self::UpdateBusinessInfo { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_calendar_event(&self) -> bool {
        matches!(self, Self::UpdateCalendarEvent { .. })
    }

    pub fn as_update_calendar_event(&self) -> Option<&UpdateCalendarEventParams> {
        match self {
            Self::UpdateCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_calendar_event(self) -> Option<UpdateCalendarEventParams> {
        match self {
            Self::UpdateCalendarEvent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_client(&self) -> bool {
        matches!(self, Self::UpdateClient { .. })
    }

    pub fn as_update_client(&self) -> Option<&UpdateClientParams> {
        match self {
            Self::UpdateClient { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_client(self) -> Option<UpdateClientParams> {
        match self {
            Self::UpdateClient { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_group_session_seats(&self) -> bool {
        matches!(self, Self::UpdateGroupSessionSeats { .. })
    }

    pub fn as_update_group_session_seats(&self) -> Option<&UpdateGroupSessionSeatsParams> {
        match self {
            Self::UpdateGroupSessionSeats { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_group_session_seats(self) -> Option<UpdateGroupSessionSeatsParams> {
        match self {
            Self::UpdateGroupSessionSeats { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_holiday(&self) -> bool {
        matches!(self, Self::UpdateHoliday { .. })
    }

    pub fn as_update_holiday(&self) -> Option<&UpdateHolidayParams> {
        match self {
            Self::UpdateHoliday { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_holiday(self) -> Option<UpdateHolidayParams> {
        match self {
            Self::UpdateHoliday { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_location(&self) -> bool {
        matches!(self, Self::UpdateLocation { .. })
    }

    pub fn as_update_location(&self) -> Option<&UpdateLocationParams> {
        match self {
            Self::UpdateLocation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_location(self) -> Option<UpdateLocationParams> {
        match self {
            Self::UpdateLocation { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_order(&self) -> bool {
        matches!(self, Self::UpdateOrder { .. })
    }

    pub fn as_update_order(&self) -> Option<&UpdateOrderParams> {
        match self {
            Self::UpdateOrder { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_order(self) -> Option<UpdateOrderParams> {
        match self {
            Self::UpdateOrder { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_product(&self) -> bool {
        matches!(self, Self::UpdateProduct { .. })
    }

    pub fn as_update_product(&self) -> Option<&UpdateProductParams> {
        match self {
            Self::UpdateProduct { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_product(self) -> Option<UpdateProductParams> {
        match self {
            Self::UpdateProduct { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_service(&self) -> bool {
        matches!(self, Self::UpdateService { .. })
    }

    pub fn as_update_service(&self) -> Option<&UpdateServiceParams> {
        match self {
            Self::UpdateService { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_service(self) -> Option<UpdateServiceParams> {
        match self {
            Self::UpdateService { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_staff(&self) -> bool {
        matches!(self, Self::UpdateStaff { .. })
    }

    pub fn as_update_staff(&self) -> Option<&UpdateStaffParams> {
        match self {
            Self::UpdateStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_staff(self) -> Option<UpdateStaffParams> {
        match self {
            Self::UpdateStaff { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_transfer_rule(&self) -> bool {
        matches!(self, Self::UpdateTransferRule { .. })
    }

    pub fn as_update_transfer_rule(&self) -> Option<&UpdateTransferRuleParams> {
        match self {
            Self::UpdateTransferRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_transfer_rule(self) -> Option<UpdateTransferRuleParams> {
        match self {
            Self::UpdateTransferRule { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_validate_user_verification_code(&self) -> bool {
        matches!(self, Self::ValidateUserVerificationCode { .. })
    }

    pub fn as_validate_user_verification_code(
        &self,
    ) -> Option<&ValidateUserVerificationCodeParams> {
        match self {
            Self::ValidateUserVerificationCode { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_validate_user_verification_code(
        self,
    ) -> Option<ValidateUserVerificationCodeParams> {
        match self {
            Self::ValidateUserVerificationCode { data } => Some(data),
            _ => None,
        }
    }
}
