use std::sync::Arc;

#[cfg(feature = "websocket")]
use crate::api::websocket::{
    SubtitlingConnector, TextToDialogueMultiContextConnector, TextToSpeechMultiContextConnector,
    TranslationConnector,
};
use crate::api::AccentsClient;
use crate::api::AgentsClient;
use crate::api::AnalysisClient;
use crate::api::AnalysisItemsClient;
use crate::api::ApiKeysClient;
use crate::api::AssetsClient;
use crate::api::AttentionClient;
use crate::api::AudioClient;
use crate::api::AudioEffectsClient;
use crate::api::AudioIsolationClient;
use crate::api::AudioNativeClient;
use crate::api::AuditLogsClient;
use crate::api::AuthConnectionsClient;
use crate::api::AvatarClient;
use crate::api::BatchCallsClient;
use crate::api::BillingClient;
use crate::api::BranchesClient;
use crate::api::BusinessProfileClient;
use crate::api::CallsClient;
use crate::api::CaptchaClient;
use crate::api::ChaptersClient;
use crate::api::ChunkClient;
use crate::api::ChunksClient;
use crate::api::CollectionsClient;
use crate::api::CompositionPlanClient;
use crate::api::ConfigClient;
use crate::api::ContentClient;
use crate::api::ContentProvenanceClient;
use crate::api::ConversationsClient;
use crate::api::CrawlJobsClient;
use crate::api::DeliverablesClient;
use crate::api::DependentAgentsClient;
use crate::api::DeploymentsClient;
use crate::api::DocumentClient;
use crate::api::DocumentsClient;
use crate::api::DraftsClient;
use crate::api::DubbingClient;
use crate::api::EnvironmentVariablesClient;
use crate::api::EnvironmentsClient;
use crate::api::ExecutionsClient;
use crate::api::ExotelClient;
use crate::api::FeedbackClient;
use crate::api::FilesClient;
use crate::api::FinetunesClient;
use crate::api::FoldersClient;
use crate::api::ForcedAlignmentClient;
use crate::api::GroupsClient;
use crate::api::HistoryClient;
use crate::api::HoldAudioClient;
use crate::api::HolidaysClient;
use crate::api::ImageClient;
use crate::api::InvitesClient;
use crate::api::InvocationsClient;
use crate::api::ItemsClient;
use crate::api::IvcClient;
use crate::api::KnowledgeBaseClient;
use crate::api::KnowledgeSourcesClient;
use crate::api::LanguageClient;
use crate::api::LanguagesClient;
use crate::api::LeadsClient;
use crate::api::LinkClient;
use crate::api::LiveCountClient;
use crate::api::LlmClient;
use crate::api::LlmUsageClient;
use crate::api::LocationsClient;
use crate::api::McpServersClient;
use crate::api::MediaClient;
use crate::api::MembersClient;
use crate::api::MergeProposalsClient;
use crate::api::MessagesClient;
use crate::api::ModelsClient;
use crate::api::MusicClient;
use crate::api::OrdersClient;
use crate::api::OutboundCampaignsClient;
use crate::api::PhoneNumbersClient;
use crate::api::PreviewClient;
use crate::api::ProceduresClient;
use crate::api::ProductsClient;
use crate::api::ProjectClient;
use crate::api::ProjectsClient;
use crate::api::PronunciationDictionariesClient;
use crate::api::PvcClient;
use crate::api::RealtimeClient;
use crate::api::ReceptionistsClient;
use crate::api::RequestsClient;
use crate::api::ResourcesClient;
use crate::api::RulesClient;
use crate::api::RunsClient;
use crate::api::SamplesClient;
use crate::api::SecretsClient;
use crate::api::ServiceAccountsClient;
use crate::api::ServicesClient;
use crate::api::SettingsClient;
use crate::api::SingleUseClient;
use crate::api::SipTrunkClient;
use crate::api::SnapshotsClient;
use crate::api::SpeakersClient;
use crate::api::SpeechEngineClient;
use crate::api::SpeechToSpeechClient;
use crate::api::SpeechToTextClient;
use crate::api::StaffClient;
use crate::api::StockClient;
use crate::api::StudioClient;
use crate::api::SubagentsClient;
use crate::api::SubscriptionClient;
use crate::api::SubscriptionsClient;
use crate::api::SummariesClient;
use crate::api::TagsClient;
use crate::api::TemplatesClient;
use crate::api::TestsClient;
use crate::api::TextToDialogueClient;
use crate::api::TextToSoundEffectsClient;
use crate::api::TextToSpeechClient;
use crate::api::TextToVoiceClient;
use crate::api::TiersClient;
use crate::api::ToolApprovalsClient;
use crate::api::ToolConfigsClient;
use crate::api::ToolsClient;
use crate::api::TopicsClient;
use crate::api::TranscriptClient;
use crate::api::TranscriptsClient;
use crate::api::TransferRulesClient;
use crate::api::TriageTicketsClient;
use crate::api::TwilioClient;
use crate::api::UsageClient;
use crate::api::UserClient;
use crate::api::UsersClient;
use crate::api::VerificationClient;
use crate::api::VersionsClient;
use crate::api::VideoClient;
use crate::api::VoicesClient;
use crate::api::WaveformClient;
use crate::api::WebhooksClient;
use crate::api::WhatsappAccountsClient;
use crate::api::WhatsappClient;
use crate::api::WidgetClient;
use crate::api::WorkspaceClient;
use crate::api::WorkspacesClient;
use crate::config::ClientConfig;
use crate::config::ClientConfigBuilder;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::environment::Environment;
use crate::error::ApiError;

pub struct ApiClient {
    pub config: Arc<ConfigManager>,
    http_client: Arc<HttpClient>,
    pub history: HistoryClient,
    pub text_to_sound_effects: TextToSoundEffectsClient,
    pub audio_isolation: AudioIsolationClient,
    pub audio_effects: AudioEffectsClient,
    pub samples: SamplesClient,
    pub audio: AudioClient,
    pub text_to_speech: TextToSpeechClient,
    pub text_to_dialogue: TextToDialogueClient,
    pub speech_to_speech: SpeechToSpeechClient,
    pub text_to_voice: TextToVoiceClient,
    pub preview: PreviewClient,
    pub user: UserClient,
    pub subscription: SubscriptionClient,
    pub collections: CollectionsClient,
    pub settings: SettingsClient,
    pub accents: AccentsClient,
    pub voices: VoicesClient,
    pub ivc: IvcClient,
    pub studio: StudioClient,
    pub pronunciation_dictionaries: PronunciationDictionariesClient,
    pub projects: ProjectsClient,
    pub content: ContentClient,
    pub snapshots: SnapshotsClient,
    pub chapters: ChaptersClient,
    pub audio_native: AudioNativeClient,
    pub project: ProjectClient,
    pub language: LanguageClient,
    pub transcript: TranscriptClient,
    pub dubbing: DubbingClient,
    pub transcripts: TranscriptsClient,
    pub models: ModelsClient,
    pub content_provenance: ContentProvenanceClient,
    pub rules: RulesClient,
    pub api_keys: ApiKeysClient,
    pub workspace: WorkspaceClient,
    pub audit_logs: AuditLogsClient,
    pub auth_connections: AuthConnectionsClient,
    pub service_accounts: ServiceAccountsClient,
    pub groups: GroupsClient,
    pub members: MembersClient,
    pub invites: InvitesClient,
    pub resources: ResourcesClient,
    pub webhooks: WebhooksClient,
    pub music: MusicClient,
    pub speech_to_text: SpeechToTextClient,
    pub single_use: SingleUseClient,
    pub forced_alignment: ForcedAlignmentClient,
    pub conversations: ConversationsClient,
    pub twilio: TwilioClient,
    pub exotel: ExotelClient,
    pub whatsapp: WhatsappClient,
    pub agents: AgentsClient,
    pub summaries: SummariesClient,
    pub subagents: SubagentsClient,
    pub widget: WidgetClient,
    pub link: LinkClient,
    pub avatar: AvatarClient,
    pub hold_audio: HoldAudioClient,
    pub knowledge_base: KnowledgeBaseClient,
    pub llm_usage: LlmUsageClient,
    pub tests: TestsClient,
    pub folders: FoldersClient,
    pub invocations: InvocationsClient,
    pub users: UsersClient,
    pub files: FilesClient,
    pub feedback: FeedbackClient,
    pub messages: MessagesClient,
    pub tags: TagsClient,
    pub triage_tickets: TriageTicketsClient,
    pub phone_numbers: PhoneNumbersClient,
    pub llm: LlmClient,
    pub live_count: LiveCountClient,
    pub documents: DocumentsClient,
    pub crawl_jobs: CrawlJobsClient,
    pub document: DocumentClient,
    pub chunk: ChunkClient,
    pub chunks: ChunksClient,
    pub topics: TopicsClient,
    pub tools: ToolsClient,
    pub executions: ExecutionsClient,
    pub analysis_items: AnalysisItemsClient,
    pub dependent_agents: DependentAgentsClient,
    pub secrets: SecretsClient,
    pub batch_calls: BatchCallsClient,
    pub sip_trunk: SipTrunkClient,
    pub mcp_servers: McpServersClient,
    pub tool_approvals: ToolApprovalsClient,
    pub tool_configs: ToolConfigsClient,
    pub whatsapp_accounts: WhatsappAccountsClient,
    pub branches: BranchesClient,
    pub versions: VersionsClient,
    pub deployments: DeploymentsClient,
    pub drafts: DraftsClient,
    pub merge_proposals: MergeProposalsClient,
    pub config_param: ConfigClient,
    pub outbound_campaigns: OutboundCampaignsClient,
    pub leads: LeadsClient,
    pub calls: CallsClient,
    pub attention: AttentionClient,
    pub speech_engine: SpeechEngineClient,
    pub procedures: ProceduresClient,
    pub analysis: AnalysisClient,
    pub environment_variables: EnvironmentVariablesClient,
    pub environments: EnvironmentsClient,
    pub realtime: RealtimeClient,
    pub composition_plan: CompositionPlanClient,
    pub finetunes: FinetunesClient,
    pub orders: OrdersClient,
    pub media: MediaClient,
    pub items: ItemsClient,
    pub deliverables: DeliverablesClient,
    pub languages: LanguagesClient,
    pub pvc: PvcClient,
    pub waveform: WaveformClient,
    pub speakers: SpeakersClient,
    pub captcha: CaptchaClient,
    pub verification: VerificationClient,
    pub video: VideoClient,
    pub image: ImageClient,
    pub runs: RunsClient,
    pub templates: TemplatesClient,
    pub assets: AssetsClient,
    pub usage: UsageClient,
    pub requests: RequestsClient,
    pub services: ServicesClient,
    pub products: ProductsClient,
    pub stock: StockClient,
    pub staff: StaffClient,
    pub receptionists: ReceptionistsClient,
    pub transfer_rules: TransferRulesClient,
    pub holidays: HolidaysClient,
    pub locations: LocationsClient,
    pub business_profile: BusinessProfileClient,
    pub knowledge_sources: KnowledgeSourcesClient,
    pub workspaces: WorkspacesClient,
    pub subscriptions: SubscriptionsClient,
    pub tiers: TiersClient,
    pub billing: BillingClient,
    #[cfg(feature = "websocket")]
    pub text_to_dialogue_multi_context: TextToDialogueMultiContextConnector,
    #[cfg(feature = "websocket")]
    pub text_to_speech_multi_context: TextToSpeechMultiContextConnector,
    #[cfg(feature = "websocket")]
    pub translation: TranslationConnector,
    #[cfg(feature = "websocket")]
    pub subtitling: SubtitlingConnector,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {
        let http_client = Arc::new(HttpClient::new(config.clone())?);
        let config_manager = Arc::new(ConfigManager::new(config));
        let history = HistoryClient::with_shared(http_client.clone(), config_manager.clone());
        let text_to_sound_effects =
            TextToSoundEffectsClient::with_shared(http_client.clone(), config_manager.clone());
        let audio_isolation =
            AudioIsolationClient::with_shared(http_client.clone(), config_manager.clone());
        let audio_effects =
            AudioEffectsClient::with_shared(http_client.clone(), config_manager.clone());
        let samples = SamplesClient::with_shared(http_client.clone(), config_manager.clone());
        let audio = AudioClient::with_shared(http_client.clone(), config_manager.clone());
        let text_to_speech =
            TextToSpeechClient::with_shared(http_client.clone(), config_manager.clone());
        let text_to_dialogue =
            TextToDialogueClient::with_shared(http_client.clone(), config_manager.clone());
        let speech_to_speech =
            SpeechToSpeechClient::with_shared(http_client.clone(), config_manager.clone());
        let text_to_voice =
            TextToVoiceClient::with_shared(http_client.clone(), config_manager.clone());
        let preview = PreviewClient::with_shared(http_client.clone(), config_manager.clone());
        let user = UserClient::with_shared(http_client.clone(), config_manager.clone());
        let subscription =
            SubscriptionClient::with_shared(http_client.clone(), config_manager.clone());
        let collections =
            CollectionsClient::with_shared(http_client.clone(), config_manager.clone());
        let settings = SettingsClient::with_shared(http_client.clone(), config_manager.clone());
        let accents = AccentsClient::with_shared(http_client.clone(), config_manager.clone());
        let voices = VoicesClient::with_shared(http_client.clone(), config_manager.clone());
        let ivc = IvcClient::with_shared(http_client.clone(), config_manager.clone());
        let studio = StudioClient::with_shared(http_client.clone(), config_manager.clone());
        let pronunciation_dictionaries = PronunciationDictionariesClient::with_shared(
            http_client.clone(),
            config_manager.clone(),
        );
        let projects = ProjectsClient::with_shared(http_client.clone(), config_manager.clone());
        let content = ContentClient::with_shared(http_client.clone(), config_manager.clone());
        let snapshots = SnapshotsClient::with_shared(http_client.clone(), config_manager.clone());
        let chapters = ChaptersClient::with_shared(http_client.clone(), config_manager.clone());
        let audio_native =
            AudioNativeClient::with_shared(http_client.clone(), config_manager.clone());
        let project = ProjectClient::with_shared(http_client.clone(), config_manager.clone());
        let language = LanguageClient::with_shared(http_client.clone(), config_manager.clone());
        let transcript = TranscriptClient::with_shared(http_client.clone(), config_manager.clone());
        let dubbing = DubbingClient::with_shared(http_client.clone(), config_manager.clone());
        let transcripts =
            TranscriptsClient::with_shared(http_client.clone(), config_manager.clone());
        let models = ModelsClient::with_shared(http_client.clone(), config_manager.clone());
        let content_provenance =
            ContentProvenanceClient::with_shared(http_client.clone(), config_manager.clone());
        let rules = RulesClient::with_shared(http_client.clone(), config_manager.clone());
        let api_keys = ApiKeysClient::with_shared(http_client.clone(), config_manager.clone());
        let workspace = WorkspaceClient::with_shared(http_client.clone(), config_manager.clone());
        let audit_logs = AuditLogsClient::with_shared(http_client.clone(), config_manager.clone());
        let auth_connections =
            AuthConnectionsClient::with_shared(http_client.clone(), config_manager.clone());
        let service_accounts =
            ServiceAccountsClient::with_shared(http_client.clone(), config_manager.clone());
        let groups = GroupsClient::with_shared(http_client.clone(), config_manager.clone());
        let members = MembersClient::with_shared(http_client.clone(), config_manager.clone());
        let invites = InvitesClient::with_shared(http_client.clone(), config_manager.clone());
        let resources = ResourcesClient::with_shared(http_client.clone(), config_manager.clone());
        let webhooks = WebhooksClient::with_shared(http_client.clone(), config_manager.clone());
        let music = MusicClient::with_shared(http_client.clone(), config_manager.clone());
        let speech_to_text =
            SpeechToTextClient::with_shared(http_client.clone(), config_manager.clone());
        let single_use = SingleUseClient::with_shared(http_client.clone(), config_manager.clone());
        let forced_alignment =
            ForcedAlignmentClient::with_shared(http_client.clone(), config_manager.clone());
        let conversations =
            ConversationsClient::with_shared(http_client.clone(), config_manager.clone());
        let twilio = TwilioClient::with_shared(http_client.clone(), config_manager.clone());
        let exotel = ExotelClient::with_shared(http_client.clone(), config_manager.clone());
        let whatsapp = WhatsappClient::with_shared(http_client.clone(), config_manager.clone());
        let agents = AgentsClient::with_shared(http_client.clone(), config_manager.clone());
        let summaries = SummariesClient::with_shared(http_client.clone(), config_manager.clone());
        let subagents = SubagentsClient::with_shared(http_client.clone(), config_manager.clone());
        let widget = WidgetClient::with_shared(http_client.clone(), config_manager.clone());
        let link = LinkClient::with_shared(http_client.clone(), config_manager.clone());
        let avatar = AvatarClient::with_shared(http_client.clone(), config_manager.clone());
        let hold_audio = HoldAudioClient::with_shared(http_client.clone(), config_manager.clone());
        let knowledge_base =
            KnowledgeBaseClient::with_shared(http_client.clone(), config_manager.clone());
        let llm_usage = LlmUsageClient::with_shared(http_client.clone(), config_manager.clone());
        let tests = TestsClient::with_shared(http_client.clone(), config_manager.clone());
        let folders = FoldersClient::with_shared(http_client.clone(), config_manager.clone());
        let invocations =
            InvocationsClient::with_shared(http_client.clone(), config_manager.clone());
        let users = UsersClient::with_shared(http_client.clone(), config_manager.clone());
        let files = FilesClient::with_shared(http_client.clone(), config_manager.clone());
        let feedback = FeedbackClient::with_shared(http_client.clone(), config_manager.clone());
        let messages = MessagesClient::with_shared(http_client.clone(), config_manager.clone());
        let tags = TagsClient::with_shared(http_client.clone(), config_manager.clone());
        let triage_tickets =
            TriageTicketsClient::with_shared(http_client.clone(), config_manager.clone());
        let phone_numbers =
            PhoneNumbersClient::with_shared(http_client.clone(), config_manager.clone());
        let llm = LlmClient::with_shared(http_client.clone(), config_manager.clone());
        let live_count = LiveCountClient::with_shared(http_client.clone(), config_manager.clone());
        let documents = DocumentsClient::with_shared(http_client.clone(), config_manager.clone());
        let crawl_jobs = CrawlJobsClient::with_shared(http_client.clone(), config_manager.clone());
        let document = DocumentClient::with_shared(http_client.clone(), config_manager.clone());
        let chunk = ChunkClient::with_shared(http_client.clone(), config_manager.clone());
        let chunks = ChunksClient::with_shared(http_client.clone(), config_manager.clone());
        let topics = TopicsClient::with_shared(http_client.clone(), config_manager.clone());
        let tools = ToolsClient::with_shared(http_client.clone(), config_manager.clone());
        let executions = ExecutionsClient::with_shared(http_client.clone(), config_manager.clone());
        let analysis_items =
            AnalysisItemsClient::with_shared(http_client.clone(), config_manager.clone());
        let dependent_agents =
            DependentAgentsClient::with_shared(http_client.clone(), config_manager.clone());
        let secrets = SecretsClient::with_shared(http_client.clone(), config_manager.clone());
        let batch_calls =
            BatchCallsClient::with_shared(http_client.clone(), config_manager.clone());
        let sip_trunk = SipTrunkClient::with_shared(http_client.clone(), config_manager.clone());
        let mcp_servers =
            McpServersClient::with_shared(http_client.clone(), config_manager.clone());
        let tool_approvals =
            ToolApprovalsClient::with_shared(http_client.clone(), config_manager.clone());
        let tool_configs =
            ToolConfigsClient::with_shared(http_client.clone(), config_manager.clone());
        let whatsapp_accounts =
            WhatsappAccountsClient::with_shared(http_client.clone(), config_manager.clone());
        let branches = BranchesClient::with_shared(http_client.clone(), config_manager.clone());
        let versions = VersionsClient::with_shared(http_client.clone(), config_manager.clone());
        let deployments =
            DeploymentsClient::with_shared(http_client.clone(), config_manager.clone());
        let drafts = DraftsClient::with_shared(http_client.clone(), config_manager.clone());
        let merge_proposals =
            MergeProposalsClient::with_shared(http_client.clone(), config_manager.clone());
        let config_param = ConfigClient::with_shared(http_client.clone(), config_manager.clone());
        let outbound_campaigns =
            OutboundCampaignsClient::with_shared(http_client.clone(), config_manager.clone());
        let leads = LeadsClient::with_shared(http_client.clone(), config_manager.clone());
        let calls = CallsClient::with_shared(http_client.clone(), config_manager.clone());
        let attention = AttentionClient::with_shared(http_client.clone(), config_manager.clone());
        let speech_engine =
            SpeechEngineClient::with_shared(http_client.clone(), config_manager.clone());
        let procedures = ProceduresClient::with_shared(http_client.clone(), config_manager.clone());
        let analysis = AnalysisClient::with_shared(http_client.clone(), config_manager.clone());
        let environment_variables =
            EnvironmentVariablesClient::with_shared(http_client.clone(), config_manager.clone());
        let environments =
            EnvironmentsClient::with_shared(http_client.clone(), config_manager.clone());
        let realtime = RealtimeClient::with_shared(http_client.clone(), config_manager.clone());
        let composition_plan =
            CompositionPlanClient::with_shared(http_client.clone(), config_manager.clone());
        let finetunes = FinetunesClient::with_shared(http_client.clone(), config_manager.clone());
        let orders = OrdersClient::with_shared(http_client.clone(), config_manager.clone());
        let media = MediaClient::with_shared(http_client.clone(), config_manager.clone());
        let items = ItemsClient::with_shared(http_client.clone(), config_manager.clone());
        let deliverables =
            DeliverablesClient::with_shared(http_client.clone(), config_manager.clone());
        let languages = LanguagesClient::with_shared(http_client.clone(), config_manager.clone());
        let pvc = PvcClient::with_shared(http_client.clone(), config_manager.clone());
        let waveform = WaveformClient::with_shared(http_client.clone(), config_manager.clone());
        let speakers = SpeakersClient::with_shared(http_client.clone(), config_manager.clone());
        let captcha = CaptchaClient::with_shared(http_client.clone(), config_manager.clone());
        let verification =
            VerificationClient::with_shared(http_client.clone(), config_manager.clone());
        let video = VideoClient::with_shared(http_client.clone(), config_manager.clone());
        let image = ImageClient::with_shared(http_client.clone(), config_manager.clone());
        let runs = RunsClient::with_shared(http_client.clone(), config_manager.clone());
        let templates = TemplatesClient::with_shared(http_client.clone(), config_manager.clone());
        let assets = AssetsClient::with_shared(http_client.clone(), config_manager.clone());
        let usage = UsageClient::with_shared(http_client.clone(), config_manager.clone());
        let requests = RequestsClient::with_shared(http_client.clone(), config_manager.clone());
        let services = ServicesClient::with_shared(http_client.clone(), config_manager.clone());
        let products = ProductsClient::with_shared(http_client.clone(), config_manager.clone());
        let stock = StockClient::with_shared(http_client.clone(), config_manager.clone());
        let staff = StaffClient::with_shared(http_client.clone(), config_manager.clone());
        let receptionists =
            ReceptionistsClient::with_shared(http_client.clone(), config_manager.clone());
        let transfer_rules =
            TransferRulesClient::with_shared(http_client.clone(), config_manager.clone());
        let holidays = HolidaysClient::with_shared(http_client.clone(), config_manager.clone());
        let locations = LocationsClient::with_shared(http_client.clone(), config_manager.clone());
        let business_profile =
            BusinessProfileClient::with_shared(http_client.clone(), config_manager.clone());
        let knowledge_sources =
            KnowledgeSourcesClient::with_shared(http_client.clone(), config_manager.clone());
        let workspaces = WorkspacesClient::with_shared(http_client.clone(), config_manager.clone());
        let subscriptions =
            SubscriptionsClient::with_shared(http_client.clone(), config_manager.clone());
        let tiers = TiersClient::with_shared(http_client.clone(), config_manager.clone());
        let billing = BillingClient::with_shared(http_client.clone(), config_manager.clone());
        #[cfg(feature = "websocket")]
        let text_to_dialogue_multi_context =
            TextToDialogueMultiContextConnector::with_shared(config_manager.clone());
        #[cfg(feature = "websocket")]
        let text_to_speech_multi_context =
            TextToSpeechMultiContextConnector::with_shared(config_manager.clone());
        #[cfg(feature = "websocket")]
        let translation = TranslationConnector::with_shared(config_manager.clone());
        #[cfg(feature = "websocket")]
        let subtitling = SubtitlingConnector::with_shared(config_manager.clone());
        Ok(Self {
            config: config_manager,
            http_client,
            history,
            text_to_sound_effects,
            audio_isolation,
            audio_effects,
            samples,
            audio,
            text_to_speech,
            text_to_dialogue,
            speech_to_speech,
            text_to_voice,
            preview,
            user,
            subscription,
            collections,
            settings,
            accents,
            voices,
            ivc,
            studio,
            pronunciation_dictionaries,
            projects,
            content,
            snapshots,
            chapters,
            audio_native,
            project,
            language,
            transcript,
            dubbing,
            transcripts,
            models,
            content_provenance,
            rules,
            api_keys,
            workspace,
            audit_logs,
            auth_connections,
            service_accounts,
            groups,
            members,
            invites,
            resources,
            webhooks,
            music,
            speech_to_text,
            single_use,
            forced_alignment,
            conversations,
            twilio,
            exotel,
            whatsapp,
            agents,
            summaries,
            subagents,
            widget,
            link,
            avatar,
            hold_audio,
            knowledge_base,
            llm_usage,
            tests,
            folders,
            invocations,
            users,
            files,
            feedback,
            messages,
            tags,
            triage_tickets,
            phone_numbers,
            llm,
            live_count,
            documents,
            crawl_jobs,
            document,
            chunk,
            chunks,
            topics,
            tools,
            executions,
            analysis_items,
            dependent_agents,
            secrets,
            batch_calls,
            sip_trunk,
            mcp_servers,
            tool_approvals,
            tool_configs,
            whatsapp_accounts,
            branches,
            versions,
            deployments,
            drafts,
            merge_proposals,
            config_param,
            outbound_campaigns,
            leads,
            calls,
            attention,
            speech_engine,
            procedures,
            analysis,
            environment_variables,
            environments,
            realtime,
            composition_plan,
            finetunes,
            orders,
            media,
            items,
            deliverables,
            languages,
            pvc,
            waveform,
            speakers,
            captcha,
            verification,
            video,
            image,
            runs,
            templates,
            assets,
            usage,
            requests,
            services,
            products,
            stock,
            staff,
            receptionists,
            transfer_rules,
            holidays,
            locations,
            business_profile,
            knowledge_sources,
            workspaces,
            subscriptions,
            tiers,
            billing,
            #[cfg(feature = "websocket")]
            text_to_dialogue_multi_context,
            #[cfg(feature = "websocket")]
            text_to_speech_multi_context,
            #[cfg(feature = "websocket")]
            translation,
            #[cfg(feature = "websocket")]
            subtitling,
        })
    }

    pub fn set_base_url(&self, base_url: impl Into<String>) -> Result<(), ApiError> {
        let url: String = base_url.into();
        self.http_client.set_base_url(url.clone());
        self.config.set_base_url(url);
        Ok(())
    }

    pub fn set_api_key(&self, key: impl Into<String>) -> Result<(), ApiError> {
        self.config.set_api_key(key);
        Ok(())
    }
}

/// Builder for creating API clients with custom configuration
pub struct ApiClientBuilder {
    config: ClientConfigBuilder,
}

impl Default for ApiClientBuilder {
    fn default() -> Self {
        Self {
            config: ClientConfig::builder(),
        }
    }
}

impl ApiClientBuilder {
    /// Create a new builder with the specified base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            config: ClientConfig::builder().base_url(base_url),
        }
    }

    /// Set the environment, updating the base URL
    pub fn environment(mut self, environment: Environment) -> Self {
        self.config = self.config.environment(environment);
        self
    }

    /// Set the API key for authentication
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.config = self.config.api_key(key);
        self
    }

    /// Set the bearer token for authentication
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.config = self.config.token(value);
        self
    }

    /// Set the username for basic authentication
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.config = self.config.username(value);
        self
    }

    /// Set the password for basic authentication
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.config = self.config.password(value);
        self
    }

    /// Set the OAuth client ID for client credentials authentication
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.config = self.config.client_id(value);
        self
    }

    /// Set the OAuth client secret for client credentials authentication
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.config = self.config.client_secret(value);
        self
    }

    /// Set OAuth credentials (client_id and client_secret) for client credentials authentication
    pub fn oauth_credentials(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        self.config = self.config.oauth_credentials(client_id, client_secret);
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config = self.config.timeout(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config = self.config.max_retries(retries);
        self
    }

    /// Add a custom header
    pub fn custom_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config = self.config.custom_header(key, value);
        self
    }

    /// Add multiple custom headers
    pub fn custom_headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.config = self.config.custom_headers(headers);
        self
    }

    /// Set the user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.config = self.config.user_agent(user_agent);
        self
    }

    /// Use a custom `reqwest` client for every request
    ///
    /// The supplied client is used as-is, so it owns all transport-level
    /// configuration — custom certificates, proxies, timeouts and user agent.
    /// Authentication, custom headers and retries are still applied by the SDK.
    pub fn reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.config = self.config.reqwest_client(client);
        self
    }

    /// Build the client with validation
    pub fn build(self) -> Result<ApiClient, crate::error::ApiError> {
        let config = self
            .config
            .build()
            .map_err(crate::error::ApiError::Configuration)?;
        ApiClient::new(config)
    }
}
