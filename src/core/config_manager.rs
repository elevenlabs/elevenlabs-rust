use crate::config::ClientConfig;
use std::sync::RwLock;
use std::time::Duration;

fn read_or_recover<T>(lock: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(|e| e.into_inner())
}

fn write_or_recover<T>(lock: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    lock.write().unwrap_or_else(|e| e.into_inner())
}

pub struct ConfigManager {
    history: RwLock<ClientConfig>,
    text_to_sound_effects: RwLock<ClientConfig>,
    audio_isolation: RwLock<ClientConfig>,
    audio_effects: RwLock<ClientConfig>,
    samples: RwLock<ClientConfig>,
    audio: RwLock<ClientConfig>,
    text_to_speech: RwLock<ClientConfig>,
    text_to_dialogue: RwLock<ClientConfig>,
    speech_to_speech: RwLock<ClientConfig>,
    text_to_voice: RwLock<ClientConfig>,
    preview: RwLock<ClientConfig>,
    user: RwLock<ClientConfig>,
    subscription: RwLock<ClientConfig>,
    collections: RwLock<ClientConfig>,
    settings: RwLock<ClientConfig>,
    accents: RwLock<ClientConfig>,
    voices: RwLock<ClientConfig>,
    ivc: RwLock<ClientConfig>,
    studio: RwLock<ClientConfig>,
    pronunciation_dictionaries: RwLock<ClientConfig>,
    projects: RwLock<ClientConfig>,
    content: RwLock<ClientConfig>,
    snapshots: RwLock<ClientConfig>,
    chapters: RwLock<ClientConfig>,
    audio_native: RwLock<ClientConfig>,
    project: RwLock<ClientConfig>,
    language: RwLock<ClientConfig>,
    transcript: RwLock<ClientConfig>,
    dubbing: RwLock<ClientConfig>,
    transcripts: RwLock<ClientConfig>,
    models: RwLock<ClientConfig>,
    content_provenance: RwLock<ClientConfig>,
    rules: RwLock<ClientConfig>,
    api_keys: RwLock<ClientConfig>,
    workspace: RwLock<ClientConfig>,
    audit_logs: RwLock<ClientConfig>,
    auth_connections: RwLock<ClientConfig>,
    service_accounts: RwLock<ClientConfig>,
    groups: RwLock<ClientConfig>,
    members: RwLock<ClientConfig>,
    invites: RwLock<ClientConfig>,
    resources: RwLock<ClientConfig>,
    webhooks: RwLock<ClientConfig>,
    music: RwLock<ClientConfig>,
    speech_to_text: RwLock<ClientConfig>,
    single_use: RwLock<ClientConfig>,
    forced_alignment: RwLock<ClientConfig>,
    conversations: RwLock<ClientConfig>,
    twilio: RwLock<ClientConfig>,
    exotel: RwLock<ClientConfig>,
    whatsapp: RwLock<ClientConfig>,
    agents: RwLock<ClientConfig>,
    summaries: RwLock<ClientConfig>,
    subagents: RwLock<ClientConfig>,
    widget: RwLock<ClientConfig>,
    link: RwLock<ClientConfig>,
    avatar: RwLock<ClientConfig>,
    hold_audio: RwLock<ClientConfig>,
    knowledge_base: RwLock<ClientConfig>,
    llm_usage: RwLock<ClientConfig>,
    tests: RwLock<ClientConfig>,
    folders: RwLock<ClientConfig>,
    invocations: RwLock<ClientConfig>,
    users: RwLock<ClientConfig>,
    files: RwLock<ClientConfig>,
    feedback: RwLock<ClientConfig>,
    messages: RwLock<ClientConfig>,
    tags: RwLock<ClientConfig>,
    triage_tickets: RwLock<ClientConfig>,
    phone_numbers: RwLock<ClientConfig>,
    llm: RwLock<ClientConfig>,
    live_count: RwLock<ClientConfig>,
    documents: RwLock<ClientConfig>,
    crawl_jobs: RwLock<ClientConfig>,
    document: RwLock<ClientConfig>,
    chunk: RwLock<ClientConfig>,
    chunks: RwLock<ClientConfig>,
    topics: RwLock<ClientConfig>,
    tools: RwLock<ClientConfig>,
    executions: RwLock<ClientConfig>,
    analysis_items: RwLock<ClientConfig>,
    dependent_agents: RwLock<ClientConfig>,
    secrets: RwLock<ClientConfig>,
    batch_calls: RwLock<ClientConfig>,
    sip_trunk: RwLock<ClientConfig>,
    mcp_servers: RwLock<ClientConfig>,
    tool_approvals: RwLock<ClientConfig>,
    tool_configs: RwLock<ClientConfig>,
    whatsapp_accounts: RwLock<ClientConfig>,
    branches: RwLock<ClientConfig>,
    versions: RwLock<ClientConfig>,
    deployments: RwLock<ClientConfig>,
    drafts: RwLock<ClientConfig>,
    merge_proposals: RwLock<ClientConfig>,
    config: RwLock<ClientConfig>,
    outbound_campaigns: RwLock<ClientConfig>,
    leads: RwLock<ClientConfig>,
    calls: RwLock<ClientConfig>,
    attention: RwLock<ClientConfig>,
    speech_engine: RwLock<ClientConfig>,
    procedures: RwLock<ClientConfig>,
    analysis: RwLock<ClientConfig>,
    environment_variables: RwLock<ClientConfig>,
    environments: RwLock<ClientConfig>,
    realtime: RwLock<ClientConfig>,
    composition_plan: RwLock<ClientConfig>,
    finetunes: RwLock<ClientConfig>,
    orders: RwLock<ClientConfig>,
    media: RwLock<ClientConfig>,
    items: RwLock<ClientConfig>,
    deliverables: RwLock<ClientConfig>,
    languages: RwLock<ClientConfig>,
    pvc: RwLock<ClientConfig>,
    waveform: RwLock<ClientConfig>,
    speakers: RwLock<ClientConfig>,
    captcha: RwLock<ClientConfig>,
    verification: RwLock<ClientConfig>,
    video: RwLock<ClientConfig>,
    image: RwLock<ClientConfig>,
    runs: RwLock<ClientConfig>,
    templates: RwLock<ClientConfig>,
    assets: RwLock<ClientConfig>,
    usage: RwLock<ClientConfig>,
    requests: RwLock<ClientConfig>,
    services: RwLock<ClientConfig>,
    products: RwLock<ClientConfig>,
    stock: RwLock<ClientConfig>,
    staff: RwLock<ClientConfig>,
    receptionists: RwLock<ClientConfig>,
    transfer_rules: RwLock<ClientConfig>,
    holidays: RwLock<ClientConfig>,
    locations: RwLock<ClientConfig>,
    business_profile: RwLock<ClientConfig>,
    knowledge_sources: RwLock<ClientConfig>,
    workspaces: RwLock<ClientConfig>,
    subscriptions: RwLock<ClientConfig>,
    tiers: RwLock<ClientConfig>,
    billing: RwLock<ClientConfig>,
    v_1_text_to_dialogue_stream_input: RwLock<ClientConfig>,
    v_1_text_to_dialogue_multi_stream_input: RwLock<ClientConfig>,
    speech_engine_upstream: RwLock<ClientConfig>,
    v_1_speech_to_text_realtime: RwLock<ClientConfig>,
    v_1_text_to_speech_voice_id_stream_input: RwLock<ClientConfig>,
    v_1_text_to_speech_voice_id_multi_stream_input: RwLock<ClientConfig>,
    v_1_translation_realtime: RwLock<ClientConfig>,
    v_1_dubbing_live: RwLock<ClientConfig>,
    v_1_subtitling_realtime: RwLock<ClientConfig>,
    v_1_convai_conversation: RwLock<ClientConfig>,
}

#[allow(dead_code)]
impl ConfigManager {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            history: RwLock::new(config.clone()),
            text_to_sound_effects: RwLock::new(config.clone()),
            audio_isolation: RwLock::new(config.clone()),
            audio_effects: RwLock::new(config.clone()),
            samples: RwLock::new(config.clone()),
            audio: RwLock::new(config.clone()),
            text_to_speech: RwLock::new(config.clone()),
            text_to_dialogue: RwLock::new(config.clone()),
            speech_to_speech: RwLock::new(config.clone()),
            text_to_voice: RwLock::new(config.clone()),
            preview: RwLock::new(config.clone()),
            user: RwLock::new(config.clone()),
            subscription: RwLock::new(config.clone()),
            collections: RwLock::new(config.clone()),
            settings: RwLock::new(config.clone()),
            accents: RwLock::new(config.clone()),
            voices: RwLock::new(config.clone()),
            ivc: RwLock::new(config.clone()),
            studio: RwLock::new(config.clone()),
            pronunciation_dictionaries: RwLock::new(config.clone()),
            projects: RwLock::new(config.clone()),
            content: RwLock::new(config.clone()),
            snapshots: RwLock::new(config.clone()),
            chapters: RwLock::new(config.clone()),
            audio_native: RwLock::new(config.clone()),
            project: RwLock::new(config.clone()),
            language: RwLock::new(config.clone()),
            transcript: RwLock::new(config.clone()),
            dubbing: RwLock::new(config.clone()),
            transcripts: RwLock::new(config.clone()),
            models: RwLock::new(config.clone()),
            content_provenance: RwLock::new(config.clone()),
            rules: RwLock::new(config.clone()),
            api_keys: RwLock::new(config.clone()),
            workspace: RwLock::new(config.clone()),
            audit_logs: RwLock::new(config.clone()),
            auth_connections: RwLock::new(config.clone()),
            service_accounts: RwLock::new(config.clone()),
            groups: RwLock::new(config.clone()),
            members: RwLock::new(config.clone()),
            invites: RwLock::new(config.clone()),
            resources: RwLock::new(config.clone()),
            webhooks: RwLock::new(config.clone()),
            music: RwLock::new(config.clone()),
            speech_to_text: RwLock::new(config.clone()),
            single_use: RwLock::new(config.clone()),
            forced_alignment: RwLock::new(config.clone()),
            conversations: RwLock::new(config.clone()),
            twilio: RwLock::new(config.clone()),
            exotel: RwLock::new(config.clone()),
            whatsapp: RwLock::new(config.clone()),
            agents: RwLock::new(config.clone()),
            summaries: RwLock::new(config.clone()),
            subagents: RwLock::new(config.clone()),
            widget: RwLock::new(config.clone()),
            link: RwLock::new(config.clone()),
            avatar: RwLock::new(config.clone()),
            hold_audio: RwLock::new(config.clone()),
            knowledge_base: RwLock::new(config.clone()),
            llm_usage: RwLock::new(config.clone()),
            tests: RwLock::new(config.clone()),
            folders: RwLock::new(config.clone()),
            invocations: RwLock::new(config.clone()),
            users: RwLock::new(config.clone()),
            files: RwLock::new(config.clone()),
            feedback: RwLock::new(config.clone()),
            messages: RwLock::new(config.clone()),
            tags: RwLock::new(config.clone()),
            triage_tickets: RwLock::new(config.clone()),
            phone_numbers: RwLock::new(config.clone()),
            llm: RwLock::new(config.clone()),
            live_count: RwLock::new(config.clone()),
            documents: RwLock::new(config.clone()),
            crawl_jobs: RwLock::new(config.clone()),
            document: RwLock::new(config.clone()),
            chunk: RwLock::new(config.clone()),
            chunks: RwLock::new(config.clone()),
            topics: RwLock::new(config.clone()),
            tools: RwLock::new(config.clone()),
            executions: RwLock::new(config.clone()),
            analysis_items: RwLock::new(config.clone()),
            dependent_agents: RwLock::new(config.clone()),
            secrets: RwLock::new(config.clone()),
            batch_calls: RwLock::new(config.clone()),
            sip_trunk: RwLock::new(config.clone()),
            mcp_servers: RwLock::new(config.clone()),
            tool_approvals: RwLock::new(config.clone()),
            tool_configs: RwLock::new(config.clone()),
            whatsapp_accounts: RwLock::new(config.clone()),
            branches: RwLock::new(config.clone()),
            versions: RwLock::new(config.clone()),
            deployments: RwLock::new(config.clone()),
            drafts: RwLock::new(config.clone()),
            merge_proposals: RwLock::new(config.clone()),
            config: RwLock::new(config.clone()),
            outbound_campaigns: RwLock::new(config.clone()),
            leads: RwLock::new(config.clone()),
            calls: RwLock::new(config.clone()),
            attention: RwLock::new(config.clone()),
            speech_engine: RwLock::new(config.clone()),
            procedures: RwLock::new(config.clone()),
            analysis: RwLock::new(config.clone()),
            environment_variables: RwLock::new(config.clone()),
            environments: RwLock::new(config.clone()),
            realtime: RwLock::new(config.clone()),
            composition_plan: RwLock::new(config.clone()),
            finetunes: RwLock::new(config.clone()),
            orders: RwLock::new(config.clone()),
            media: RwLock::new(config.clone()),
            items: RwLock::new(config.clone()),
            deliverables: RwLock::new(config.clone()),
            languages: RwLock::new(config.clone()),
            pvc: RwLock::new(config.clone()),
            waveform: RwLock::new(config.clone()),
            speakers: RwLock::new(config.clone()),
            captcha: RwLock::new(config.clone()),
            verification: RwLock::new(config.clone()),
            video: RwLock::new(config.clone()),
            image: RwLock::new(config.clone()),
            runs: RwLock::new(config.clone()),
            templates: RwLock::new(config.clone()),
            assets: RwLock::new(config.clone()),
            usage: RwLock::new(config.clone()),
            requests: RwLock::new(config.clone()),
            services: RwLock::new(config.clone()),
            products: RwLock::new(config.clone()),
            stock: RwLock::new(config.clone()),
            staff: RwLock::new(config.clone()),
            receptionists: RwLock::new(config.clone()),
            transfer_rules: RwLock::new(config.clone()),
            holidays: RwLock::new(config.clone()),
            locations: RwLock::new(config.clone()),
            business_profile: RwLock::new(config.clone()),
            knowledge_sources: RwLock::new(config.clone()),
            workspaces: RwLock::new(config.clone()),
            subscriptions: RwLock::new(config.clone()),
            tiers: RwLock::new(config.clone()),
            billing: RwLock::new(config.clone()),
            v_1_text_to_dialogue_stream_input: RwLock::new(config.clone()),
            v_1_text_to_dialogue_multi_stream_input: RwLock::new(config.clone()),
            speech_engine_upstream: RwLock::new(config.clone()),
            v_1_speech_to_text_realtime: RwLock::new(config.clone()),
            v_1_text_to_speech_voice_id_stream_input: RwLock::new(config.clone()),
            v_1_text_to_speech_voice_id_multi_stream_input: RwLock::new(config.clone()),
            v_1_translation_realtime: RwLock::new(config.clone()),
            v_1_dubbing_live: RwLock::new(config.clone()),
            v_1_subtitling_realtime: RwLock::new(config.clone()),
            v_1_convai_conversation: RwLock::new(config),
        }
    }

    pub fn get_history_config(&self) -> ClientConfig {
        read_or_recover(&self.history).clone()
    }

    pub fn get_text_to_sound_effects_config(&self) -> ClientConfig {
        read_or_recover(&self.text_to_sound_effects).clone()
    }

    pub fn get_audio_isolation_config(&self) -> ClientConfig {
        read_or_recover(&self.audio_isolation).clone()
    }

    pub fn get_audio_effects_config(&self) -> ClientConfig {
        read_or_recover(&self.audio_effects).clone()
    }

    pub fn get_samples_config(&self) -> ClientConfig {
        read_or_recover(&self.samples).clone()
    }

    pub fn get_audio_config(&self) -> ClientConfig {
        read_or_recover(&self.audio).clone()
    }

    pub fn get_text_to_speech_config(&self) -> ClientConfig {
        read_or_recover(&self.text_to_speech).clone()
    }

    pub fn get_text_to_dialogue_config(&self) -> ClientConfig {
        read_or_recover(&self.text_to_dialogue).clone()
    }

    pub fn get_speech_to_speech_config(&self) -> ClientConfig {
        read_or_recover(&self.speech_to_speech).clone()
    }

    pub fn get_text_to_voice_config(&self) -> ClientConfig {
        read_or_recover(&self.text_to_voice).clone()
    }

    pub fn get_preview_config(&self) -> ClientConfig {
        read_or_recover(&self.preview).clone()
    }

    pub fn get_user_config(&self) -> ClientConfig {
        read_or_recover(&self.user).clone()
    }

    pub fn get_subscription_config(&self) -> ClientConfig {
        read_or_recover(&self.subscription).clone()
    }

    pub fn get_collections_config(&self) -> ClientConfig {
        read_or_recover(&self.collections).clone()
    }

    pub fn get_settings_config(&self) -> ClientConfig {
        read_or_recover(&self.settings).clone()
    }

    pub fn get_accents_config(&self) -> ClientConfig {
        read_or_recover(&self.accents).clone()
    }

    pub fn get_voices_config(&self) -> ClientConfig {
        read_or_recover(&self.voices).clone()
    }

    pub fn get_ivc_config(&self) -> ClientConfig {
        read_or_recover(&self.ivc).clone()
    }

    pub fn get_studio_config(&self) -> ClientConfig {
        read_or_recover(&self.studio).clone()
    }

    pub fn get_pronunciation_dictionaries_config(&self) -> ClientConfig {
        read_or_recover(&self.pronunciation_dictionaries).clone()
    }

    pub fn get_projects_config(&self) -> ClientConfig {
        read_or_recover(&self.projects).clone()
    }

    pub fn get_content_config(&self) -> ClientConfig {
        read_or_recover(&self.content).clone()
    }

    pub fn get_snapshots_config(&self) -> ClientConfig {
        read_or_recover(&self.snapshots).clone()
    }

    pub fn get_chapters_config(&self) -> ClientConfig {
        read_or_recover(&self.chapters).clone()
    }

    pub fn get_audio_native_config(&self) -> ClientConfig {
        read_or_recover(&self.audio_native).clone()
    }

    pub fn get_project_config(&self) -> ClientConfig {
        read_or_recover(&self.project).clone()
    }

    pub fn get_language_config(&self) -> ClientConfig {
        read_or_recover(&self.language).clone()
    }

    pub fn get_transcript_config(&self) -> ClientConfig {
        read_or_recover(&self.transcript).clone()
    }

    pub fn get_dubbing_config(&self) -> ClientConfig {
        read_or_recover(&self.dubbing).clone()
    }

    pub fn get_transcripts_config(&self) -> ClientConfig {
        read_or_recover(&self.transcripts).clone()
    }

    pub fn get_models_config(&self) -> ClientConfig {
        read_or_recover(&self.models).clone()
    }

    pub fn get_content_provenance_config(&self) -> ClientConfig {
        read_or_recover(&self.content_provenance).clone()
    }

    pub fn get_rules_config(&self) -> ClientConfig {
        read_or_recover(&self.rules).clone()
    }

    pub fn get_api_keys_config(&self) -> ClientConfig {
        read_or_recover(&self.api_keys).clone()
    }

    pub fn get_workspace_config(&self) -> ClientConfig {
        read_or_recover(&self.workspace).clone()
    }

    pub fn get_audit_logs_config(&self) -> ClientConfig {
        read_or_recover(&self.audit_logs).clone()
    }

    pub fn get_auth_connections_config(&self) -> ClientConfig {
        read_or_recover(&self.auth_connections).clone()
    }

    pub fn get_service_accounts_config(&self) -> ClientConfig {
        read_or_recover(&self.service_accounts).clone()
    }

    pub fn get_groups_config(&self) -> ClientConfig {
        read_or_recover(&self.groups).clone()
    }

    pub fn get_members_config(&self) -> ClientConfig {
        read_or_recover(&self.members).clone()
    }

    pub fn get_invites_config(&self) -> ClientConfig {
        read_or_recover(&self.invites).clone()
    }

    pub fn get_resources_config(&self) -> ClientConfig {
        read_or_recover(&self.resources).clone()
    }

    pub fn get_webhooks_config(&self) -> ClientConfig {
        read_or_recover(&self.webhooks).clone()
    }

    pub fn get_music_config(&self) -> ClientConfig {
        read_or_recover(&self.music).clone()
    }

    pub fn get_speech_to_text_config(&self) -> ClientConfig {
        read_or_recover(&self.speech_to_text).clone()
    }

    pub fn get_single_use_config(&self) -> ClientConfig {
        read_or_recover(&self.single_use).clone()
    }

    pub fn get_forced_alignment_config(&self) -> ClientConfig {
        read_or_recover(&self.forced_alignment).clone()
    }

    pub fn get_conversations_config(&self) -> ClientConfig {
        read_or_recover(&self.conversations).clone()
    }

    pub fn get_twilio_config(&self) -> ClientConfig {
        read_or_recover(&self.twilio).clone()
    }

    pub fn get_exotel_config(&self) -> ClientConfig {
        read_or_recover(&self.exotel).clone()
    }

    pub fn get_whatsapp_config(&self) -> ClientConfig {
        read_or_recover(&self.whatsapp).clone()
    }

    pub fn get_agents_config(&self) -> ClientConfig {
        read_or_recover(&self.agents).clone()
    }

    pub fn get_summaries_config(&self) -> ClientConfig {
        read_or_recover(&self.summaries).clone()
    }

    pub fn get_subagents_config(&self) -> ClientConfig {
        read_or_recover(&self.subagents).clone()
    }

    pub fn get_widget_config(&self) -> ClientConfig {
        read_or_recover(&self.widget).clone()
    }

    pub fn get_link_config(&self) -> ClientConfig {
        read_or_recover(&self.link).clone()
    }

    pub fn get_avatar_config(&self) -> ClientConfig {
        read_or_recover(&self.avatar).clone()
    }

    pub fn get_hold_audio_config(&self) -> ClientConfig {
        read_or_recover(&self.hold_audio).clone()
    }

    pub fn get_knowledge_base_config(&self) -> ClientConfig {
        read_or_recover(&self.knowledge_base).clone()
    }

    pub fn get_llm_usage_config(&self) -> ClientConfig {
        read_or_recover(&self.llm_usage).clone()
    }

    pub fn get_tests_config(&self) -> ClientConfig {
        read_or_recover(&self.tests).clone()
    }

    pub fn get_folders_config(&self) -> ClientConfig {
        read_or_recover(&self.folders).clone()
    }

    pub fn get_invocations_config(&self) -> ClientConfig {
        read_or_recover(&self.invocations).clone()
    }

    pub fn get_users_config(&self) -> ClientConfig {
        read_or_recover(&self.users).clone()
    }

    pub fn get_files_config(&self) -> ClientConfig {
        read_or_recover(&self.files).clone()
    }

    pub fn get_feedback_config(&self) -> ClientConfig {
        read_or_recover(&self.feedback).clone()
    }

    pub fn get_messages_config(&self) -> ClientConfig {
        read_or_recover(&self.messages).clone()
    }

    pub fn get_tags_config(&self) -> ClientConfig {
        read_or_recover(&self.tags).clone()
    }

    pub fn get_triage_tickets_config(&self) -> ClientConfig {
        read_or_recover(&self.triage_tickets).clone()
    }

    pub fn get_phone_numbers_config(&self) -> ClientConfig {
        read_or_recover(&self.phone_numbers).clone()
    }

    pub fn get_llm_config(&self) -> ClientConfig {
        read_or_recover(&self.llm).clone()
    }

    pub fn get_live_count_config(&self) -> ClientConfig {
        read_or_recover(&self.live_count).clone()
    }

    pub fn get_documents_config(&self) -> ClientConfig {
        read_or_recover(&self.documents).clone()
    }

    pub fn get_crawl_jobs_config(&self) -> ClientConfig {
        read_or_recover(&self.crawl_jobs).clone()
    }

    pub fn get_document_config(&self) -> ClientConfig {
        read_or_recover(&self.document).clone()
    }

    pub fn get_chunk_config(&self) -> ClientConfig {
        read_or_recover(&self.chunk).clone()
    }

    pub fn get_chunks_config(&self) -> ClientConfig {
        read_or_recover(&self.chunks).clone()
    }

    pub fn get_topics_config(&self) -> ClientConfig {
        read_or_recover(&self.topics).clone()
    }

    pub fn get_tools_config(&self) -> ClientConfig {
        read_or_recover(&self.tools).clone()
    }

    pub fn get_executions_config(&self) -> ClientConfig {
        read_or_recover(&self.executions).clone()
    }

    pub fn get_analysis_items_config(&self) -> ClientConfig {
        read_or_recover(&self.analysis_items).clone()
    }

    pub fn get_dependent_agents_config(&self) -> ClientConfig {
        read_or_recover(&self.dependent_agents).clone()
    }

    pub fn get_secrets_config(&self) -> ClientConfig {
        read_or_recover(&self.secrets).clone()
    }

    pub fn get_batch_calls_config(&self) -> ClientConfig {
        read_or_recover(&self.batch_calls).clone()
    }

    pub fn get_sip_trunk_config(&self) -> ClientConfig {
        read_or_recover(&self.sip_trunk).clone()
    }

    pub fn get_mcp_servers_config(&self) -> ClientConfig {
        read_or_recover(&self.mcp_servers).clone()
    }

    pub fn get_tool_approvals_config(&self) -> ClientConfig {
        read_or_recover(&self.tool_approvals).clone()
    }

    pub fn get_tool_configs_config(&self) -> ClientConfig {
        read_or_recover(&self.tool_configs).clone()
    }

    pub fn get_whatsapp_accounts_config(&self) -> ClientConfig {
        read_or_recover(&self.whatsapp_accounts).clone()
    }

    pub fn get_branches_config(&self) -> ClientConfig {
        read_or_recover(&self.branches).clone()
    }

    pub fn get_versions_config(&self) -> ClientConfig {
        read_or_recover(&self.versions).clone()
    }

    pub fn get_deployments_config(&self) -> ClientConfig {
        read_or_recover(&self.deployments).clone()
    }

    pub fn get_drafts_config(&self) -> ClientConfig {
        read_or_recover(&self.drafts).clone()
    }

    pub fn get_merge_proposals_config(&self) -> ClientConfig {
        read_or_recover(&self.merge_proposals).clone()
    }

    pub fn get_config_config(&self) -> ClientConfig {
        read_or_recover(&self.config).clone()
    }

    pub fn get_outbound_campaigns_config(&self) -> ClientConfig {
        read_or_recover(&self.outbound_campaigns).clone()
    }

    pub fn get_leads_config(&self) -> ClientConfig {
        read_or_recover(&self.leads).clone()
    }

    pub fn get_calls_config(&self) -> ClientConfig {
        read_or_recover(&self.calls).clone()
    }

    pub fn get_attention_config(&self) -> ClientConfig {
        read_or_recover(&self.attention).clone()
    }

    pub fn get_speech_engine_config(&self) -> ClientConfig {
        read_or_recover(&self.speech_engine).clone()
    }

    pub fn get_procedures_config(&self) -> ClientConfig {
        read_or_recover(&self.procedures).clone()
    }

    pub fn get_analysis_config(&self) -> ClientConfig {
        read_or_recover(&self.analysis).clone()
    }

    pub fn get_environment_variables_config(&self) -> ClientConfig {
        read_or_recover(&self.environment_variables).clone()
    }

    pub fn get_environments_config(&self) -> ClientConfig {
        read_or_recover(&self.environments).clone()
    }

    pub fn get_realtime_config(&self) -> ClientConfig {
        read_or_recover(&self.realtime).clone()
    }

    pub fn get_composition_plan_config(&self) -> ClientConfig {
        read_or_recover(&self.composition_plan).clone()
    }

    pub fn get_finetunes_config(&self) -> ClientConfig {
        read_or_recover(&self.finetunes).clone()
    }

    pub fn get_orders_config(&self) -> ClientConfig {
        read_or_recover(&self.orders).clone()
    }

    pub fn get_media_config(&self) -> ClientConfig {
        read_or_recover(&self.media).clone()
    }

    pub fn get_items_config(&self) -> ClientConfig {
        read_or_recover(&self.items).clone()
    }

    pub fn get_deliverables_config(&self) -> ClientConfig {
        read_or_recover(&self.deliverables).clone()
    }

    pub fn get_languages_config(&self) -> ClientConfig {
        read_or_recover(&self.languages).clone()
    }

    pub fn get_pvc_config(&self) -> ClientConfig {
        read_or_recover(&self.pvc).clone()
    }

    pub fn get_waveform_config(&self) -> ClientConfig {
        read_or_recover(&self.waveform).clone()
    }

    pub fn get_speakers_config(&self) -> ClientConfig {
        read_or_recover(&self.speakers).clone()
    }

    pub fn get_captcha_config(&self) -> ClientConfig {
        read_or_recover(&self.captcha).clone()
    }

    pub fn get_verification_config(&self) -> ClientConfig {
        read_or_recover(&self.verification).clone()
    }

    pub fn get_video_config(&self) -> ClientConfig {
        read_or_recover(&self.video).clone()
    }

    pub fn get_image_config(&self) -> ClientConfig {
        read_or_recover(&self.image).clone()
    }

    pub fn get_runs_config(&self) -> ClientConfig {
        read_or_recover(&self.runs).clone()
    }

    pub fn get_templates_config(&self) -> ClientConfig {
        read_or_recover(&self.templates).clone()
    }

    pub fn get_assets_config(&self) -> ClientConfig {
        read_or_recover(&self.assets).clone()
    }

    pub fn get_usage_config(&self) -> ClientConfig {
        read_or_recover(&self.usage).clone()
    }

    pub fn get_requests_config(&self) -> ClientConfig {
        read_or_recover(&self.requests).clone()
    }

    pub fn get_services_config(&self) -> ClientConfig {
        read_or_recover(&self.services).clone()
    }

    pub fn get_products_config(&self) -> ClientConfig {
        read_or_recover(&self.products).clone()
    }

    pub fn get_stock_config(&self) -> ClientConfig {
        read_or_recover(&self.stock).clone()
    }

    pub fn get_staff_config(&self) -> ClientConfig {
        read_or_recover(&self.staff).clone()
    }

    pub fn get_receptionists_config(&self) -> ClientConfig {
        read_or_recover(&self.receptionists).clone()
    }

    pub fn get_transfer_rules_config(&self) -> ClientConfig {
        read_or_recover(&self.transfer_rules).clone()
    }

    pub fn get_holidays_config(&self) -> ClientConfig {
        read_or_recover(&self.holidays).clone()
    }

    pub fn get_locations_config(&self) -> ClientConfig {
        read_or_recover(&self.locations).clone()
    }

    pub fn get_business_profile_config(&self) -> ClientConfig {
        read_or_recover(&self.business_profile).clone()
    }

    pub fn get_knowledge_sources_config(&self) -> ClientConfig {
        read_or_recover(&self.knowledge_sources).clone()
    }

    pub fn get_workspaces_config(&self) -> ClientConfig {
        read_or_recover(&self.workspaces).clone()
    }

    pub fn get_subscriptions_config(&self) -> ClientConfig {
        read_or_recover(&self.subscriptions).clone()
    }

    pub fn get_tiers_config(&self) -> ClientConfig {
        read_or_recover(&self.tiers).clone()
    }

    pub fn get_billing_config(&self) -> ClientConfig {
        read_or_recover(&self.billing).clone()
    }

    pub fn get_v_1_text_to_dialogue_stream_input_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_text_to_dialogue_stream_input).clone()
    }

    pub fn get_v_1_text_to_dialogue_multi_stream_input_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).clone()
    }

    pub fn get_speech_engine_upstream_config(&self) -> ClientConfig {
        read_or_recover(&self.speech_engine_upstream).clone()
    }

    pub fn get_v_1_speech_to_text_realtime_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_speech_to_text_realtime).clone()
    }

    pub fn get_v_1_text_to_speech_voice_id_stream_input_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).clone()
    }

    pub fn get_v_1_text_to_speech_voice_id_multi_stream_input_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).clone()
    }

    pub fn get_v_1_translation_realtime_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_translation_realtime).clone()
    }

    pub fn get_v_1_dubbing_live_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_dubbing_live).clone()
    }

    pub fn get_v_1_subtitling_realtime_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_subtitling_realtime).clone()
    }

    pub fn get_v_1_convai_conversation_config(&self) -> ClientConfig {
        read_or_recover(&self.v_1_convai_conversation).clone()
    }

    pub fn set_base_url(&self, base_url: impl Into<String>) {
        let val = base_url.into();
        write_or_recover(&self.history).base_url = val.clone();
        write_or_recover(&self.text_to_sound_effects).base_url = val.clone();
        write_or_recover(&self.audio_isolation).base_url = val.clone();
        write_or_recover(&self.audio_effects).base_url = val.clone();
        write_or_recover(&self.samples).base_url = val.clone();
        write_or_recover(&self.audio).base_url = val.clone();
        write_or_recover(&self.text_to_speech).base_url = val.clone();
        write_or_recover(&self.text_to_dialogue).base_url = val.clone();
        write_or_recover(&self.speech_to_speech).base_url = val.clone();
        write_or_recover(&self.text_to_voice).base_url = val.clone();
        write_or_recover(&self.preview).base_url = val.clone();
        write_or_recover(&self.user).base_url = val.clone();
        write_or_recover(&self.subscription).base_url = val.clone();
        write_or_recover(&self.collections).base_url = val.clone();
        write_or_recover(&self.settings).base_url = val.clone();
        write_or_recover(&self.accents).base_url = val.clone();
        write_or_recover(&self.voices).base_url = val.clone();
        write_or_recover(&self.ivc).base_url = val.clone();
        write_or_recover(&self.studio).base_url = val.clone();
        write_or_recover(&self.pronunciation_dictionaries).base_url = val.clone();
        write_or_recover(&self.projects).base_url = val.clone();
        write_or_recover(&self.content).base_url = val.clone();
        write_or_recover(&self.snapshots).base_url = val.clone();
        write_or_recover(&self.chapters).base_url = val.clone();
        write_or_recover(&self.audio_native).base_url = val.clone();
        write_or_recover(&self.project).base_url = val.clone();
        write_or_recover(&self.language).base_url = val.clone();
        write_or_recover(&self.transcript).base_url = val.clone();
        write_or_recover(&self.dubbing).base_url = val.clone();
        write_or_recover(&self.transcripts).base_url = val.clone();
        write_or_recover(&self.models).base_url = val.clone();
        write_or_recover(&self.content_provenance).base_url = val.clone();
        write_or_recover(&self.rules).base_url = val.clone();
        write_or_recover(&self.api_keys).base_url = val.clone();
        write_or_recover(&self.workspace).base_url = val.clone();
        write_or_recover(&self.audit_logs).base_url = val.clone();
        write_or_recover(&self.auth_connections).base_url = val.clone();
        write_or_recover(&self.service_accounts).base_url = val.clone();
        write_or_recover(&self.groups).base_url = val.clone();
        write_or_recover(&self.members).base_url = val.clone();
        write_or_recover(&self.invites).base_url = val.clone();
        write_or_recover(&self.resources).base_url = val.clone();
        write_or_recover(&self.webhooks).base_url = val.clone();
        write_or_recover(&self.music).base_url = val.clone();
        write_or_recover(&self.speech_to_text).base_url = val.clone();
        write_or_recover(&self.single_use).base_url = val.clone();
        write_or_recover(&self.forced_alignment).base_url = val.clone();
        write_or_recover(&self.conversations).base_url = val.clone();
        write_or_recover(&self.twilio).base_url = val.clone();
        write_or_recover(&self.exotel).base_url = val.clone();
        write_or_recover(&self.whatsapp).base_url = val.clone();
        write_or_recover(&self.agents).base_url = val.clone();
        write_or_recover(&self.summaries).base_url = val.clone();
        write_or_recover(&self.subagents).base_url = val.clone();
        write_or_recover(&self.widget).base_url = val.clone();
        write_or_recover(&self.link).base_url = val.clone();
        write_or_recover(&self.avatar).base_url = val.clone();
        write_or_recover(&self.hold_audio).base_url = val.clone();
        write_or_recover(&self.knowledge_base).base_url = val.clone();
        write_or_recover(&self.llm_usage).base_url = val.clone();
        write_or_recover(&self.tests).base_url = val.clone();
        write_or_recover(&self.folders).base_url = val.clone();
        write_or_recover(&self.invocations).base_url = val.clone();
        write_or_recover(&self.users).base_url = val.clone();
        write_or_recover(&self.files).base_url = val.clone();
        write_or_recover(&self.feedback).base_url = val.clone();
        write_or_recover(&self.messages).base_url = val.clone();
        write_or_recover(&self.tags).base_url = val.clone();
        write_or_recover(&self.triage_tickets).base_url = val.clone();
        write_or_recover(&self.phone_numbers).base_url = val.clone();
        write_or_recover(&self.llm).base_url = val.clone();
        write_or_recover(&self.live_count).base_url = val.clone();
        write_or_recover(&self.documents).base_url = val.clone();
        write_or_recover(&self.crawl_jobs).base_url = val.clone();
        write_or_recover(&self.document).base_url = val.clone();
        write_or_recover(&self.chunk).base_url = val.clone();
        write_or_recover(&self.chunks).base_url = val.clone();
        write_or_recover(&self.topics).base_url = val.clone();
        write_or_recover(&self.tools).base_url = val.clone();
        write_or_recover(&self.executions).base_url = val.clone();
        write_or_recover(&self.analysis_items).base_url = val.clone();
        write_or_recover(&self.dependent_agents).base_url = val.clone();
        write_or_recover(&self.secrets).base_url = val.clone();
        write_or_recover(&self.batch_calls).base_url = val.clone();
        write_or_recover(&self.sip_trunk).base_url = val.clone();
        write_or_recover(&self.mcp_servers).base_url = val.clone();
        write_or_recover(&self.tool_approvals).base_url = val.clone();
        write_or_recover(&self.tool_configs).base_url = val.clone();
        write_or_recover(&self.whatsapp_accounts).base_url = val.clone();
        write_or_recover(&self.branches).base_url = val.clone();
        write_or_recover(&self.versions).base_url = val.clone();
        write_or_recover(&self.deployments).base_url = val.clone();
        write_or_recover(&self.drafts).base_url = val.clone();
        write_or_recover(&self.merge_proposals).base_url = val.clone();
        write_or_recover(&self.config).base_url = val.clone();
        write_or_recover(&self.outbound_campaigns).base_url = val.clone();
        write_or_recover(&self.leads).base_url = val.clone();
        write_or_recover(&self.calls).base_url = val.clone();
        write_or_recover(&self.attention).base_url = val.clone();
        write_or_recover(&self.speech_engine).base_url = val.clone();
        write_or_recover(&self.procedures).base_url = val.clone();
        write_or_recover(&self.analysis).base_url = val.clone();
        write_or_recover(&self.environment_variables).base_url = val.clone();
        write_or_recover(&self.environments).base_url = val.clone();
        write_or_recover(&self.realtime).base_url = val.clone();
        write_or_recover(&self.composition_plan).base_url = val.clone();
        write_or_recover(&self.finetunes).base_url = val.clone();
        write_or_recover(&self.orders).base_url = val.clone();
        write_or_recover(&self.media).base_url = val.clone();
        write_or_recover(&self.items).base_url = val.clone();
        write_or_recover(&self.deliverables).base_url = val.clone();
        write_or_recover(&self.languages).base_url = val.clone();
        write_or_recover(&self.pvc).base_url = val.clone();
        write_or_recover(&self.waveform).base_url = val.clone();
        write_or_recover(&self.speakers).base_url = val.clone();
        write_or_recover(&self.captcha).base_url = val.clone();
        write_or_recover(&self.verification).base_url = val.clone();
        write_or_recover(&self.video).base_url = val.clone();
        write_or_recover(&self.image).base_url = val.clone();
        write_or_recover(&self.runs).base_url = val.clone();
        write_or_recover(&self.templates).base_url = val.clone();
        write_or_recover(&self.assets).base_url = val.clone();
        write_or_recover(&self.usage).base_url = val.clone();
        write_or_recover(&self.requests).base_url = val.clone();
        write_or_recover(&self.services).base_url = val.clone();
        write_or_recover(&self.products).base_url = val.clone();
        write_or_recover(&self.stock).base_url = val.clone();
        write_or_recover(&self.staff).base_url = val.clone();
        write_or_recover(&self.receptionists).base_url = val.clone();
        write_or_recover(&self.transfer_rules).base_url = val.clone();
        write_or_recover(&self.holidays).base_url = val.clone();
        write_or_recover(&self.locations).base_url = val.clone();
        write_or_recover(&self.business_profile).base_url = val.clone();
        write_or_recover(&self.knowledge_sources).base_url = val.clone();
        write_or_recover(&self.workspaces).base_url = val.clone();
        write_or_recover(&self.subscriptions).base_url = val.clone();
        write_or_recover(&self.tiers).base_url = val.clone();
        write_or_recover(&self.billing).base_url = val.clone();
        write_or_recover(&self.v_1_text_to_dialogue_stream_input).base_url = val.clone();
        write_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).base_url = val.clone();
        write_or_recover(&self.speech_engine_upstream).base_url = val.clone();
        write_or_recover(&self.v_1_speech_to_text_realtime).base_url = val.clone();
        write_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).base_url = val.clone();
        write_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).base_url =
            val.clone();
        write_or_recover(&self.v_1_translation_realtime).base_url = val.clone();
        write_or_recover(&self.v_1_dubbing_live).base_url = val.clone();
        write_or_recover(&self.v_1_subtitling_realtime).base_url = val.clone();
        write_or_recover(&self.v_1_convai_conversation).base_url = val.clone();
    }

    pub fn set_timeout(&self, timeout: Duration) {
        let val = timeout;
        write_or_recover(&self.history).timeout = val.clone();
        write_or_recover(&self.text_to_sound_effects).timeout = val.clone();
        write_or_recover(&self.audio_isolation).timeout = val.clone();
        write_or_recover(&self.audio_effects).timeout = val.clone();
        write_or_recover(&self.samples).timeout = val.clone();
        write_or_recover(&self.audio).timeout = val.clone();
        write_or_recover(&self.text_to_speech).timeout = val.clone();
        write_or_recover(&self.text_to_dialogue).timeout = val.clone();
        write_or_recover(&self.speech_to_speech).timeout = val.clone();
        write_or_recover(&self.text_to_voice).timeout = val.clone();
        write_or_recover(&self.preview).timeout = val.clone();
        write_or_recover(&self.user).timeout = val.clone();
        write_or_recover(&self.subscription).timeout = val.clone();
        write_or_recover(&self.collections).timeout = val.clone();
        write_or_recover(&self.settings).timeout = val.clone();
        write_or_recover(&self.accents).timeout = val.clone();
        write_or_recover(&self.voices).timeout = val.clone();
        write_or_recover(&self.ivc).timeout = val.clone();
        write_or_recover(&self.studio).timeout = val.clone();
        write_or_recover(&self.pronunciation_dictionaries).timeout = val.clone();
        write_or_recover(&self.projects).timeout = val.clone();
        write_or_recover(&self.content).timeout = val.clone();
        write_or_recover(&self.snapshots).timeout = val.clone();
        write_or_recover(&self.chapters).timeout = val.clone();
        write_or_recover(&self.audio_native).timeout = val.clone();
        write_or_recover(&self.project).timeout = val.clone();
        write_or_recover(&self.language).timeout = val.clone();
        write_or_recover(&self.transcript).timeout = val.clone();
        write_or_recover(&self.dubbing).timeout = val.clone();
        write_or_recover(&self.transcripts).timeout = val.clone();
        write_or_recover(&self.models).timeout = val.clone();
        write_or_recover(&self.content_provenance).timeout = val.clone();
        write_or_recover(&self.rules).timeout = val.clone();
        write_or_recover(&self.api_keys).timeout = val.clone();
        write_or_recover(&self.workspace).timeout = val.clone();
        write_or_recover(&self.audit_logs).timeout = val.clone();
        write_or_recover(&self.auth_connections).timeout = val.clone();
        write_or_recover(&self.service_accounts).timeout = val.clone();
        write_or_recover(&self.groups).timeout = val.clone();
        write_or_recover(&self.members).timeout = val.clone();
        write_or_recover(&self.invites).timeout = val.clone();
        write_or_recover(&self.resources).timeout = val.clone();
        write_or_recover(&self.webhooks).timeout = val.clone();
        write_or_recover(&self.music).timeout = val.clone();
        write_or_recover(&self.speech_to_text).timeout = val.clone();
        write_or_recover(&self.single_use).timeout = val.clone();
        write_or_recover(&self.forced_alignment).timeout = val.clone();
        write_or_recover(&self.conversations).timeout = val.clone();
        write_or_recover(&self.twilio).timeout = val.clone();
        write_or_recover(&self.exotel).timeout = val.clone();
        write_or_recover(&self.whatsapp).timeout = val.clone();
        write_or_recover(&self.agents).timeout = val.clone();
        write_or_recover(&self.summaries).timeout = val.clone();
        write_or_recover(&self.subagents).timeout = val.clone();
        write_or_recover(&self.widget).timeout = val.clone();
        write_or_recover(&self.link).timeout = val.clone();
        write_or_recover(&self.avatar).timeout = val.clone();
        write_or_recover(&self.hold_audio).timeout = val.clone();
        write_or_recover(&self.knowledge_base).timeout = val.clone();
        write_or_recover(&self.llm_usage).timeout = val.clone();
        write_or_recover(&self.tests).timeout = val.clone();
        write_or_recover(&self.folders).timeout = val.clone();
        write_or_recover(&self.invocations).timeout = val.clone();
        write_or_recover(&self.users).timeout = val.clone();
        write_or_recover(&self.files).timeout = val.clone();
        write_or_recover(&self.feedback).timeout = val.clone();
        write_or_recover(&self.messages).timeout = val.clone();
        write_or_recover(&self.tags).timeout = val.clone();
        write_or_recover(&self.triage_tickets).timeout = val.clone();
        write_or_recover(&self.phone_numbers).timeout = val.clone();
        write_or_recover(&self.llm).timeout = val.clone();
        write_or_recover(&self.live_count).timeout = val.clone();
        write_or_recover(&self.documents).timeout = val.clone();
        write_or_recover(&self.crawl_jobs).timeout = val.clone();
        write_or_recover(&self.document).timeout = val.clone();
        write_or_recover(&self.chunk).timeout = val.clone();
        write_or_recover(&self.chunks).timeout = val.clone();
        write_or_recover(&self.topics).timeout = val.clone();
        write_or_recover(&self.tools).timeout = val.clone();
        write_or_recover(&self.executions).timeout = val.clone();
        write_or_recover(&self.analysis_items).timeout = val.clone();
        write_or_recover(&self.dependent_agents).timeout = val.clone();
        write_or_recover(&self.secrets).timeout = val.clone();
        write_or_recover(&self.batch_calls).timeout = val.clone();
        write_or_recover(&self.sip_trunk).timeout = val.clone();
        write_or_recover(&self.mcp_servers).timeout = val.clone();
        write_or_recover(&self.tool_approvals).timeout = val.clone();
        write_or_recover(&self.tool_configs).timeout = val.clone();
        write_or_recover(&self.whatsapp_accounts).timeout = val.clone();
        write_or_recover(&self.branches).timeout = val.clone();
        write_or_recover(&self.versions).timeout = val.clone();
        write_or_recover(&self.deployments).timeout = val.clone();
        write_or_recover(&self.drafts).timeout = val.clone();
        write_or_recover(&self.merge_proposals).timeout = val.clone();
        write_or_recover(&self.config).timeout = val.clone();
        write_or_recover(&self.outbound_campaigns).timeout = val.clone();
        write_or_recover(&self.leads).timeout = val.clone();
        write_or_recover(&self.calls).timeout = val.clone();
        write_or_recover(&self.attention).timeout = val.clone();
        write_or_recover(&self.speech_engine).timeout = val.clone();
        write_or_recover(&self.procedures).timeout = val.clone();
        write_or_recover(&self.analysis).timeout = val.clone();
        write_or_recover(&self.environment_variables).timeout = val.clone();
        write_or_recover(&self.environments).timeout = val.clone();
        write_or_recover(&self.realtime).timeout = val.clone();
        write_or_recover(&self.composition_plan).timeout = val.clone();
        write_or_recover(&self.finetunes).timeout = val.clone();
        write_or_recover(&self.orders).timeout = val.clone();
        write_or_recover(&self.media).timeout = val.clone();
        write_or_recover(&self.items).timeout = val.clone();
        write_or_recover(&self.deliverables).timeout = val.clone();
        write_or_recover(&self.languages).timeout = val.clone();
        write_or_recover(&self.pvc).timeout = val.clone();
        write_or_recover(&self.waveform).timeout = val.clone();
        write_or_recover(&self.speakers).timeout = val.clone();
        write_or_recover(&self.captcha).timeout = val.clone();
        write_or_recover(&self.verification).timeout = val.clone();
        write_or_recover(&self.video).timeout = val.clone();
        write_or_recover(&self.image).timeout = val.clone();
        write_or_recover(&self.runs).timeout = val.clone();
        write_or_recover(&self.templates).timeout = val.clone();
        write_or_recover(&self.assets).timeout = val.clone();
        write_or_recover(&self.usage).timeout = val.clone();
        write_or_recover(&self.requests).timeout = val.clone();
        write_or_recover(&self.services).timeout = val.clone();
        write_or_recover(&self.products).timeout = val.clone();
        write_or_recover(&self.stock).timeout = val.clone();
        write_or_recover(&self.staff).timeout = val.clone();
        write_or_recover(&self.receptionists).timeout = val.clone();
        write_or_recover(&self.transfer_rules).timeout = val.clone();
        write_or_recover(&self.holidays).timeout = val.clone();
        write_or_recover(&self.locations).timeout = val.clone();
        write_or_recover(&self.business_profile).timeout = val.clone();
        write_or_recover(&self.knowledge_sources).timeout = val.clone();
        write_or_recover(&self.workspaces).timeout = val.clone();
        write_or_recover(&self.subscriptions).timeout = val.clone();
        write_or_recover(&self.tiers).timeout = val.clone();
        write_or_recover(&self.billing).timeout = val.clone();
        write_or_recover(&self.v_1_text_to_dialogue_stream_input).timeout = val.clone();
        write_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).timeout = val.clone();
        write_or_recover(&self.speech_engine_upstream).timeout = val.clone();
        write_or_recover(&self.v_1_speech_to_text_realtime).timeout = val.clone();
        write_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).timeout = val.clone();
        write_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).timeout =
            val.clone();
        write_or_recover(&self.v_1_translation_realtime).timeout = val.clone();
        write_or_recover(&self.v_1_dubbing_live).timeout = val.clone();
        write_or_recover(&self.v_1_subtitling_realtime).timeout = val.clone();
        write_or_recover(&self.v_1_convai_conversation).timeout = val.clone();
    }

    pub fn set_api_key(&self, api_key: impl Into<String>) {
        let val = Some(api_key.into());
        write_or_recover(&self.history).api_key = val.clone();
        write_or_recover(&self.text_to_sound_effects).api_key = val.clone();
        write_or_recover(&self.audio_isolation).api_key = val.clone();
        write_or_recover(&self.audio_effects).api_key = val.clone();
        write_or_recover(&self.samples).api_key = val.clone();
        write_or_recover(&self.audio).api_key = val.clone();
        write_or_recover(&self.text_to_speech).api_key = val.clone();
        write_or_recover(&self.text_to_dialogue).api_key = val.clone();
        write_or_recover(&self.speech_to_speech).api_key = val.clone();
        write_or_recover(&self.text_to_voice).api_key = val.clone();
        write_or_recover(&self.preview).api_key = val.clone();
        write_or_recover(&self.user).api_key = val.clone();
        write_or_recover(&self.subscription).api_key = val.clone();
        write_or_recover(&self.collections).api_key = val.clone();
        write_or_recover(&self.settings).api_key = val.clone();
        write_or_recover(&self.accents).api_key = val.clone();
        write_or_recover(&self.voices).api_key = val.clone();
        write_or_recover(&self.ivc).api_key = val.clone();
        write_or_recover(&self.studio).api_key = val.clone();
        write_or_recover(&self.pronunciation_dictionaries).api_key = val.clone();
        write_or_recover(&self.projects).api_key = val.clone();
        write_or_recover(&self.content).api_key = val.clone();
        write_or_recover(&self.snapshots).api_key = val.clone();
        write_or_recover(&self.chapters).api_key = val.clone();
        write_or_recover(&self.audio_native).api_key = val.clone();
        write_or_recover(&self.project).api_key = val.clone();
        write_or_recover(&self.language).api_key = val.clone();
        write_or_recover(&self.transcript).api_key = val.clone();
        write_or_recover(&self.dubbing).api_key = val.clone();
        write_or_recover(&self.transcripts).api_key = val.clone();
        write_or_recover(&self.models).api_key = val.clone();
        write_or_recover(&self.content_provenance).api_key = val.clone();
        write_or_recover(&self.rules).api_key = val.clone();
        write_or_recover(&self.api_keys).api_key = val.clone();
        write_or_recover(&self.workspace).api_key = val.clone();
        write_or_recover(&self.audit_logs).api_key = val.clone();
        write_or_recover(&self.auth_connections).api_key = val.clone();
        write_or_recover(&self.service_accounts).api_key = val.clone();
        write_or_recover(&self.groups).api_key = val.clone();
        write_or_recover(&self.members).api_key = val.clone();
        write_or_recover(&self.invites).api_key = val.clone();
        write_or_recover(&self.resources).api_key = val.clone();
        write_or_recover(&self.webhooks).api_key = val.clone();
        write_or_recover(&self.music).api_key = val.clone();
        write_or_recover(&self.speech_to_text).api_key = val.clone();
        write_or_recover(&self.single_use).api_key = val.clone();
        write_or_recover(&self.forced_alignment).api_key = val.clone();
        write_or_recover(&self.conversations).api_key = val.clone();
        write_or_recover(&self.twilio).api_key = val.clone();
        write_or_recover(&self.exotel).api_key = val.clone();
        write_or_recover(&self.whatsapp).api_key = val.clone();
        write_or_recover(&self.agents).api_key = val.clone();
        write_or_recover(&self.summaries).api_key = val.clone();
        write_or_recover(&self.subagents).api_key = val.clone();
        write_or_recover(&self.widget).api_key = val.clone();
        write_or_recover(&self.link).api_key = val.clone();
        write_or_recover(&self.avatar).api_key = val.clone();
        write_or_recover(&self.hold_audio).api_key = val.clone();
        write_or_recover(&self.knowledge_base).api_key = val.clone();
        write_or_recover(&self.llm_usage).api_key = val.clone();
        write_or_recover(&self.tests).api_key = val.clone();
        write_or_recover(&self.folders).api_key = val.clone();
        write_or_recover(&self.invocations).api_key = val.clone();
        write_or_recover(&self.users).api_key = val.clone();
        write_or_recover(&self.files).api_key = val.clone();
        write_or_recover(&self.feedback).api_key = val.clone();
        write_or_recover(&self.messages).api_key = val.clone();
        write_or_recover(&self.tags).api_key = val.clone();
        write_or_recover(&self.triage_tickets).api_key = val.clone();
        write_or_recover(&self.phone_numbers).api_key = val.clone();
        write_or_recover(&self.llm).api_key = val.clone();
        write_or_recover(&self.live_count).api_key = val.clone();
        write_or_recover(&self.documents).api_key = val.clone();
        write_or_recover(&self.crawl_jobs).api_key = val.clone();
        write_or_recover(&self.document).api_key = val.clone();
        write_or_recover(&self.chunk).api_key = val.clone();
        write_or_recover(&self.chunks).api_key = val.clone();
        write_or_recover(&self.topics).api_key = val.clone();
        write_or_recover(&self.tools).api_key = val.clone();
        write_or_recover(&self.executions).api_key = val.clone();
        write_or_recover(&self.analysis_items).api_key = val.clone();
        write_or_recover(&self.dependent_agents).api_key = val.clone();
        write_or_recover(&self.secrets).api_key = val.clone();
        write_or_recover(&self.batch_calls).api_key = val.clone();
        write_or_recover(&self.sip_trunk).api_key = val.clone();
        write_or_recover(&self.mcp_servers).api_key = val.clone();
        write_or_recover(&self.tool_approvals).api_key = val.clone();
        write_or_recover(&self.tool_configs).api_key = val.clone();
        write_or_recover(&self.whatsapp_accounts).api_key = val.clone();
        write_or_recover(&self.branches).api_key = val.clone();
        write_or_recover(&self.versions).api_key = val.clone();
        write_or_recover(&self.deployments).api_key = val.clone();
        write_or_recover(&self.drafts).api_key = val.clone();
        write_or_recover(&self.merge_proposals).api_key = val.clone();
        write_or_recover(&self.config).api_key = val.clone();
        write_or_recover(&self.outbound_campaigns).api_key = val.clone();
        write_or_recover(&self.leads).api_key = val.clone();
        write_or_recover(&self.calls).api_key = val.clone();
        write_or_recover(&self.attention).api_key = val.clone();
        write_or_recover(&self.speech_engine).api_key = val.clone();
        write_or_recover(&self.procedures).api_key = val.clone();
        write_or_recover(&self.analysis).api_key = val.clone();
        write_or_recover(&self.environment_variables).api_key = val.clone();
        write_or_recover(&self.environments).api_key = val.clone();
        write_or_recover(&self.realtime).api_key = val.clone();
        write_or_recover(&self.composition_plan).api_key = val.clone();
        write_or_recover(&self.finetunes).api_key = val.clone();
        write_or_recover(&self.orders).api_key = val.clone();
        write_or_recover(&self.media).api_key = val.clone();
        write_or_recover(&self.items).api_key = val.clone();
        write_or_recover(&self.deliverables).api_key = val.clone();
        write_or_recover(&self.languages).api_key = val.clone();
        write_or_recover(&self.pvc).api_key = val.clone();
        write_or_recover(&self.waveform).api_key = val.clone();
        write_or_recover(&self.speakers).api_key = val.clone();
        write_or_recover(&self.captcha).api_key = val.clone();
        write_or_recover(&self.verification).api_key = val.clone();
        write_or_recover(&self.video).api_key = val.clone();
        write_or_recover(&self.image).api_key = val.clone();
        write_or_recover(&self.runs).api_key = val.clone();
        write_or_recover(&self.templates).api_key = val.clone();
        write_or_recover(&self.assets).api_key = val.clone();
        write_or_recover(&self.usage).api_key = val.clone();
        write_or_recover(&self.requests).api_key = val.clone();
        write_or_recover(&self.services).api_key = val.clone();
        write_or_recover(&self.products).api_key = val.clone();
        write_or_recover(&self.stock).api_key = val.clone();
        write_or_recover(&self.staff).api_key = val.clone();
        write_or_recover(&self.receptionists).api_key = val.clone();
        write_or_recover(&self.transfer_rules).api_key = val.clone();
        write_or_recover(&self.holidays).api_key = val.clone();
        write_or_recover(&self.locations).api_key = val.clone();
        write_or_recover(&self.business_profile).api_key = val.clone();
        write_or_recover(&self.knowledge_sources).api_key = val.clone();
        write_or_recover(&self.workspaces).api_key = val.clone();
        write_or_recover(&self.subscriptions).api_key = val.clone();
        write_or_recover(&self.tiers).api_key = val.clone();
        write_or_recover(&self.billing).api_key = val.clone();
        write_or_recover(&self.v_1_text_to_dialogue_stream_input).api_key = val.clone();
        write_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).api_key = val.clone();
        write_or_recover(&self.speech_engine_upstream).api_key = val.clone();
        write_or_recover(&self.v_1_speech_to_text_realtime).api_key = val.clone();
        write_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).api_key = val.clone();
        write_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).api_key =
            val.clone();
        write_or_recover(&self.v_1_translation_realtime).api_key = val.clone();
        write_or_recover(&self.v_1_dubbing_live).api_key = val.clone();
        write_or_recover(&self.v_1_subtitling_realtime).api_key = val.clone();
        write_or_recover(&self.v_1_convai_conversation).api_key = val.clone();
    }

    pub(crate) fn set_history_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.history).base_url = base_url.into();
    }

    pub(crate) fn set_history_timeout(&self, timeout: Duration) {
        write_or_recover(&self.history).timeout = timeout;
    }

    pub(crate) fn set_history_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.history).api_key = Some(api_key.into());
    }

    pub(crate) fn set_text_to_sound_effects_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.text_to_sound_effects).base_url = base_url.into();
    }

    pub(crate) fn set_text_to_sound_effects_timeout(&self, timeout: Duration) {
        write_or_recover(&self.text_to_sound_effects).timeout = timeout;
    }

    pub(crate) fn set_text_to_sound_effects_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.text_to_sound_effects).api_key = Some(api_key.into());
    }

    pub(crate) fn set_audio_isolation_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.audio_isolation).base_url = base_url.into();
    }

    pub(crate) fn set_audio_isolation_timeout(&self, timeout: Duration) {
        write_or_recover(&self.audio_isolation).timeout = timeout;
    }

    pub(crate) fn set_audio_isolation_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.audio_isolation).api_key = Some(api_key.into());
    }

    pub(crate) fn set_audio_effects_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.audio_effects).base_url = base_url.into();
    }

    pub(crate) fn set_audio_effects_timeout(&self, timeout: Duration) {
        write_or_recover(&self.audio_effects).timeout = timeout;
    }

    pub(crate) fn set_audio_effects_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.audio_effects).api_key = Some(api_key.into());
    }

    pub(crate) fn set_samples_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.samples).base_url = base_url.into();
    }

    pub(crate) fn set_samples_timeout(&self, timeout: Duration) {
        write_or_recover(&self.samples).timeout = timeout;
    }

    pub(crate) fn set_samples_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.samples).api_key = Some(api_key.into());
    }

    pub(crate) fn set_audio_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.audio).base_url = base_url.into();
    }

    pub(crate) fn set_audio_timeout(&self, timeout: Duration) {
        write_or_recover(&self.audio).timeout = timeout;
    }

    pub(crate) fn set_audio_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.audio).api_key = Some(api_key.into());
    }

    pub(crate) fn set_text_to_speech_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.text_to_speech).base_url = base_url.into();
    }

    pub(crate) fn set_text_to_speech_timeout(&self, timeout: Duration) {
        write_or_recover(&self.text_to_speech).timeout = timeout;
    }

    pub(crate) fn set_text_to_speech_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.text_to_speech).api_key = Some(api_key.into());
    }

    pub(crate) fn set_text_to_dialogue_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.text_to_dialogue).base_url = base_url.into();
    }

    pub(crate) fn set_text_to_dialogue_timeout(&self, timeout: Duration) {
        write_or_recover(&self.text_to_dialogue).timeout = timeout;
    }

    pub(crate) fn set_text_to_dialogue_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.text_to_dialogue).api_key = Some(api_key.into());
    }

    pub(crate) fn set_speech_to_speech_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.speech_to_speech).base_url = base_url.into();
    }

    pub(crate) fn set_speech_to_speech_timeout(&self, timeout: Duration) {
        write_or_recover(&self.speech_to_speech).timeout = timeout;
    }

    pub(crate) fn set_speech_to_speech_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.speech_to_speech).api_key = Some(api_key.into());
    }

    pub(crate) fn set_text_to_voice_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.text_to_voice).base_url = base_url.into();
    }

    pub(crate) fn set_text_to_voice_timeout(&self, timeout: Duration) {
        write_or_recover(&self.text_to_voice).timeout = timeout;
    }

    pub(crate) fn set_text_to_voice_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.text_to_voice).api_key = Some(api_key.into());
    }

    pub(crate) fn set_preview_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.preview).base_url = base_url.into();
    }

    pub(crate) fn set_preview_timeout(&self, timeout: Duration) {
        write_or_recover(&self.preview).timeout = timeout;
    }

    pub(crate) fn set_preview_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.preview).api_key = Some(api_key.into());
    }

    pub(crate) fn set_user_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.user).base_url = base_url.into();
    }

    pub(crate) fn set_user_timeout(&self, timeout: Duration) {
        write_or_recover(&self.user).timeout = timeout;
    }

    pub(crate) fn set_user_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.user).api_key = Some(api_key.into());
    }

    pub(crate) fn set_subscription_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.subscription).base_url = base_url.into();
    }

    pub(crate) fn set_subscription_timeout(&self, timeout: Duration) {
        write_or_recover(&self.subscription).timeout = timeout;
    }

    pub(crate) fn set_subscription_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.subscription).api_key = Some(api_key.into());
    }

    pub(crate) fn set_collections_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.collections).base_url = base_url.into();
    }

    pub(crate) fn set_collections_timeout(&self, timeout: Duration) {
        write_or_recover(&self.collections).timeout = timeout;
    }

    pub(crate) fn set_collections_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.collections).api_key = Some(api_key.into());
    }

    pub(crate) fn set_settings_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.settings).base_url = base_url.into();
    }

    pub(crate) fn set_settings_timeout(&self, timeout: Duration) {
        write_or_recover(&self.settings).timeout = timeout;
    }

    pub(crate) fn set_settings_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.settings).api_key = Some(api_key.into());
    }

    pub(crate) fn set_accents_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.accents).base_url = base_url.into();
    }

    pub(crate) fn set_accents_timeout(&self, timeout: Duration) {
        write_or_recover(&self.accents).timeout = timeout;
    }

    pub(crate) fn set_accents_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.accents).api_key = Some(api_key.into());
    }

    pub(crate) fn set_voices_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.voices).base_url = base_url.into();
    }

    pub(crate) fn set_voices_timeout(&self, timeout: Duration) {
        write_or_recover(&self.voices).timeout = timeout;
    }

    pub(crate) fn set_voices_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.voices).api_key = Some(api_key.into());
    }

    pub(crate) fn set_ivc_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.ivc).base_url = base_url.into();
    }

    pub(crate) fn set_ivc_timeout(&self, timeout: Duration) {
        write_or_recover(&self.ivc).timeout = timeout;
    }

    pub(crate) fn set_ivc_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.ivc).api_key = Some(api_key.into());
    }

    pub(crate) fn set_studio_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.studio).base_url = base_url.into();
    }

    pub(crate) fn set_studio_timeout(&self, timeout: Duration) {
        write_or_recover(&self.studio).timeout = timeout;
    }

    pub(crate) fn set_studio_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.studio).api_key = Some(api_key.into());
    }

    pub(crate) fn set_pronunciation_dictionaries_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.pronunciation_dictionaries).base_url = base_url.into();
    }

    pub(crate) fn set_pronunciation_dictionaries_timeout(&self, timeout: Duration) {
        write_or_recover(&self.pronunciation_dictionaries).timeout = timeout;
    }

    pub(crate) fn set_pronunciation_dictionaries_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.pronunciation_dictionaries).api_key = Some(api_key.into());
    }

    pub(crate) fn set_projects_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.projects).base_url = base_url.into();
    }

    pub(crate) fn set_projects_timeout(&self, timeout: Duration) {
        write_or_recover(&self.projects).timeout = timeout;
    }

    pub(crate) fn set_projects_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.projects).api_key = Some(api_key.into());
    }

    pub(crate) fn set_content_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.content).base_url = base_url.into();
    }

    pub(crate) fn set_content_timeout(&self, timeout: Duration) {
        write_or_recover(&self.content).timeout = timeout;
    }

    pub(crate) fn set_content_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.content).api_key = Some(api_key.into());
    }

    pub(crate) fn set_snapshots_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.snapshots).base_url = base_url.into();
    }

    pub(crate) fn set_snapshots_timeout(&self, timeout: Duration) {
        write_or_recover(&self.snapshots).timeout = timeout;
    }

    pub(crate) fn set_snapshots_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.snapshots).api_key = Some(api_key.into());
    }

    pub(crate) fn set_chapters_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.chapters).base_url = base_url.into();
    }

    pub(crate) fn set_chapters_timeout(&self, timeout: Duration) {
        write_or_recover(&self.chapters).timeout = timeout;
    }

    pub(crate) fn set_chapters_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.chapters).api_key = Some(api_key.into());
    }

    pub(crate) fn set_audio_native_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.audio_native).base_url = base_url.into();
    }

    pub(crate) fn set_audio_native_timeout(&self, timeout: Duration) {
        write_or_recover(&self.audio_native).timeout = timeout;
    }

    pub(crate) fn set_audio_native_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.audio_native).api_key = Some(api_key.into());
    }

    pub(crate) fn set_project_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.project).base_url = base_url.into();
    }

    pub(crate) fn set_project_timeout(&self, timeout: Duration) {
        write_or_recover(&self.project).timeout = timeout;
    }

    pub(crate) fn set_project_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.project).api_key = Some(api_key.into());
    }

    pub(crate) fn set_language_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.language).base_url = base_url.into();
    }

    pub(crate) fn set_language_timeout(&self, timeout: Duration) {
        write_or_recover(&self.language).timeout = timeout;
    }

    pub(crate) fn set_language_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.language).api_key = Some(api_key.into());
    }

    pub(crate) fn set_transcript_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.transcript).base_url = base_url.into();
    }

    pub(crate) fn set_transcript_timeout(&self, timeout: Duration) {
        write_or_recover(&self.transcript).timeout = timeout;
    }

    pub(crate) fn set_transcript_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.transcript).api_key = Some(api_key.into());
    }

    pub(crate) fn set_dubbing_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.dubbing).base_url = base_url.into();
    }

    pub(crate) fn set_dubbing_timeout(&self, timeout: Duration) {
        write_or_recover(&self.dubbing).timeout = timeout;
    }

    pub(crate) fn set_dubbing_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.dubbing).api_key = Some(api_key.into());
    }

    pub(crate) fn set_transcripts_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.transcripts).base_url = base_url.into();
    }

    pub(crate) fn set_transcripts_timeout(&self, timeout: Duration) {
        write_or_recover(&self.transcripts).timeout = timeout;
    }

    pub(crate) fn set_transcripts_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.transcripts).api_key = Some(api_key.into());
    }

    pub(crate) fn set_models_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.models).base_url = base_url.into();
    }

    pub(crate) fn set_models_timeout(&self, timeout: Duration) {
        write_or_recover(&self.models).timeout = timeout;
    }

    pub(crate) fn set_models_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.models).api_key = Some(api_key.into());
    }

    pub(crate) fn set_content_provenance_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.content_provenance).base_url = base_url.into();
    }

    pub(crate) fn set_content_provenance_timeout(&self, timeout: Duration) {
        write_or_recover(&self.content_provenance).timeout = timeout;
    }

    pub(crate) fn set_content_provenance_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.content_provenance).api_key = Some(api_key.into());
    }

    pub(crate) fn set_rules_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.rules).base_url = base_url.into();
    }

    pub(crate) fn set_rules_timeout(&self, timeout: Duration) {
        write_or_recover(&self.rules).timeout = timeout;
    }

    pub(crate) fn set_rules_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.rules).api_key = Some(api_key.into());
    }

    pub(crate) fn set_api_keys_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.api_keys).base_url = base_url.into();
    }

    pub(crate) fn set_api_keys_timeout(&self, timeout: Duration) {
        write_or_recover(&self.api_keys).timeout = timeout;
    }

    pub(crate) fn set_api_keys_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.api_keys).api_key = Some(api_key.into());
    }

    pub(crate) fn set_workspace_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.workspace).base_url = base_url.into();
    }

    pub(crate) fn set_workspace_timeout(&self, timeout: Duration) {
        write_or_recover(&self.workspace).timeout = timeout;
    }

    pub(crate) fn set_workspace_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.workspace).api_key = Some(api_key.into());
    }

    pub(crate) fn set_audit_logs_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.audit_logs).base_url = base_url.into();
    }

    pub(crate) fn set_audit_logs_timeout(&self, timeout: Duration) {
        write_or_recover(&self.audit_logs).timeout = timeout;
    }

    pub(crate) fn set_audit_logs_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.audit_logs).api_key = Some(api_key.into());
    }

    pub(crate) fn set_auth_connections_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.auth_connections).base_url = base_url.into();
    }

    pub(crate) fn set_auth_connections_timeout(&self, timeout: Duration) {
        write_or_recover(&self.auth_connections).timeout = timeout;
    }

    pub(crate) fn set_auth_connections_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.auth_connections).api_key = Some(api_key.into());
    }

    pub(crate) fn set_service_accounts_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.service_accounts).base_url = base_url.into();
    }

    pub(crate) fn set_service_accounts_timeout(&self, timeout: Duration) {
        write_or_recover(&self.service_accounts).timeout = timeout;
    }

    pub(crate) fn set_service_accounts_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.service_accounts).api_key = Some(api_key.into());
    }

    pub(crate) fn set_groups_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.groups).base_url = base_url.into();
    }

    pub(crate) fn set_groups_timeout(&self, timeout: Duration) {
        write_or_recover(&self.groups).timeout = timeout;
    }

    pub(crate) fn set_groups_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.groups).api_key = Some(api_key.into());
    }

    pub(crate) fn set_members_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.members).base_url = base_url.into();
    }

    pub(crate) fn set_members_timeout(&self, timeout: Duration) {
        write_or_recover(&self.members).timeout = timeout;
    }

    pub(crate) fn set_members_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.members).api_key = Some(api_key.into());
    }

    pub(crate) fn set_invites_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.invites).base_url = base_url.into();
    }

    pub(crate) fn set_invites_timeout(&self, timeout: Duration) {
        write_or_recover(&self.invites).timeout = timeout;
    }

    pub(crate) fn set_invites_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.invites).api_key = Some(api_key.into());
    }

    pub(crate) fn set_resources_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.resources).base_url = base_url.into();
    }

    pub(crate) fn set_resources_timeout(&self, timeout: Duration) {
        write_or_recover(&self.resources).timeout = timeout;
    }

    pub(crate) fn set_resources_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.resources).api_key = Some(api_key.into());
    }

    pub(crate) fn set_webhooks_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.webhooks).base_url = base_url.into();
    }

    pub(crate) fn set_webhooks_timeout(&self, timeout: Duration) {
        write_or_recover(&self.webhooks).timeout = timeout;
    }

    pub(crate) fn set_webhooks_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.webhooks).api_key = Some(api_key.into());
    }

    pub(crate) fn set_music_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.music).base_url = base_url.into();
    }

    pub(crate) fn set_music_timeout(&self, timeout: Duration) {
        write_or_recover(&self.music).timeout = timeout;
    }

    pub(crate) fn set_music_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.music).api_key = Some(api_key.into());
    }

    pub(crate) fn set_speech_to_text_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.speech_to_text).base_url = base_url.into();
    }

    pub(crate) fn set_speech_to_text_timeout(&self, timeout: Duration) {
        write_or_recover(&self.speech_to_text).timeout = timeout;
    }

    pub(crate) fn set_speech_to_text_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.speech_to_text).api_key = Some(api_key.into());
    }

    pub(crate) fn set_single_use_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.single_use).base_url = base_url.into();
    }

    pub(crate) fn set_single_use_timeout(&self, timeout: Duration) {
        write_or_recover(&self.single_use).timeout = timeout;
    }

    pub(crate) fn set_single_use_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.single_use).api_key = Some(api_key.into());
    }

    pub(crate) fn set_forced_alignment_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.forced_alignment).base_url = base_url.into();
    }

    pub(crate) fn set_forced_alignment_timeout(&self, timeout: Duration) {
        write_or_recover(&self.forced_alignment).timeout = timeout;
    }

    pub(crate) fn set_forced_alignment_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.forced_alignment).api_key = Some(api_key.into());
    }

    pub(crate) fn set_conversations_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.conversations).base_url = base_url.into();
    }

    pub(crate) fn set_conversations_timeout(&self, timeout: Duration) {
        write_or_recover(&self.conversations).timeout = timeout;
    }

    pub(crate) fn set_conversations_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.conversations).api_key = Some(api_key.into());
    }

    pub(crate) fn set_twilio_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.twilio).base_url = base_url.into();
    }

    pub(crate) fn set_twilio_timeout(&self, timeout: Duration) {
        write_or_recover(&self.twilio).timeout = timeout;
    }

    pub(crate) fn set_twilio_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.twilio).api_key = Some(api_key.into());
    }

    pub(crate) fn set_exotel_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.exotel).base_url = base_url.into();
    }

    pub(crate) fn set_exotel_timeout(&self, timeout: Duration) {
        write_or_recover(&self.exotel).timeout = timeout;
    }

    pub(crate) fn set_exotel_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.exotel).api_key = Some(api_key.into());
    }

    pub(crate) fn set_whatsapp_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.whatsapp).base_url = base_url.into();
    }

    pub(crate) fn set_whatsapp_timeout(&self, timeout: Duration) {
        write_or_recover(&self.whatsapp).timeout = timeout;
    }

    pub(crate) fn set_whatsapp_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.whatsapp).api_key = Some(api_key.into());
    }

    pub(crate) fn set_agents_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.agents).base_url = base_url.into();
    }

    pub(crate) fn set_agents_timeout(&self, timeout: Duration) {
        write_or_recover(&self.agents).timeout = timeout;
    }

    pub(crate) fn set_agents_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.agents).api_key = Some(api_key.into());
    }

    pub(crate) fn set_summaries_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.summaries).base_url = base_url.into();
    }

    pub(crate) fn set_summaries_timeout(&self, timeout: Duration) {
        write_or_recover(&self.summaries).timeout = timeout;
    }

    pub(crate) fn set_summaries_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.summaries).api_key = Some(api_key.into());
    }

    pub(crate) fn set_subagents_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.subagents).base_url = base_url.into();
    }

    pub(crate) fn set_subagents_timeout(&self, timeout: Duration) {
        write_or_recover(&self.subagents).timeout = timeout;
    }

    pub(crate) fn set_subagents_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.subagents).api_key = Some(api_key.into());
    }

    pub(crate) fn set_widget_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.widget).base_url = base_url.into();
    }

    pub(crate) fn set_widget_timeout(&self, timeout: Duration) {
        write_or_recover(&self.widget).timeout = timeout;
    }

    pub(crate) fn set_widget_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.widget).api_key = Some(api_key.into());
    }

    pub(crate) fn set_link_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.link).base_url = base_url.into();
    }

    pub(crate) fn set_link_timeout(&self, timeout: Duration) {
        write_or_recover(&self.link).timeout = timeout;
    }

    pub(crate) fn set_link_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.link).api_key = Some(api_key.into());
    }

    pub(crate) fn set_avatar_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.avatar).base_url = base_url.into();
    }

    pub(crate) fn set_avatar_timeout(&self, timeout: Duration) {
        write_or_recover(&self.avatar).timeout = timeout;
    }

    pub(crate) fn set_avatar_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.avatar).api_key = Some(api_key.into());
    }

    pub(crate) fn set_hold_audio_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.hold_audio).base_url = base_url.into();
    }

    pub(crate) fn set_hold_audio_timeout(&self, timeout: Duration) {
        write_or_recover(&self.hold_audio).timeout = timeout;
    }

    pub(crate) fn set_hold_audio_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.hold_audio).api_key = Some(api_key.into());
    }

    pub(crate) fn set_knowledge_base_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.knowledge_base).base_url = base_url.into();
    }

    pub(crate) fn set_knowledge_base_timeout(&self, timeout: Duration) {
        write_or_recover(&self.knowledge_base).timeout = timeout;
    }

    pub(crate) fn set_knowledge_base_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.knowledge_base).api_key = Some(api_key.into());
    }

    pub(crate) fn set_llm_usage_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.llm_usage).base_url = base_url.into();
    }

    pub(crate) fn set_llm_usage_timeout(&self, timeout: Duration) {
        write_or_recover(&self.llm_usage).timeout = timeout;
    }

    pub(crate) fn set_llm_usage_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.llm_usage).api_key = Some(api_key.into());
    }

    pub(crate) fn set_tests_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.tests).base_url = base_url.into();
    }

    pub(crate) fn set_tests_timeout(&self, timeout: Duration) {
        write_or_recover(&self.tests).timeout = timeout;
    }

    pub(crate) fn set_tests_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.tests).api_key = Some(api_key.into());
    }

    pub(crate) fn set_folders_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.folders).base_url = base_url.into();
    }

    pub(crate) fn set_folders_timeout(&self, timeout: Duration) {
        write_or_recover(&self.folders).timeout = timeout;
    }

    pub(crate) fn set_folders_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.folders).api_key = Some(api_key.into());
    }

    pub(crate) fn set_invocations_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.invocations).base_url = base_url.into();
    }

    pub(crate) fn set_invocations_timeout(&self, timeout: Duration) {
        write_or_recover(&self.invocations).timeout = timeout;
    }

    pub(crate) fn set_invocations_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.invocations).api_key = Some(api_key.into());
    }

    pub(crate) fn set_users_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.users).base_url = base_url.into();
    }

    pub(crate) fn set_users_timeout(&self, timeout: Duration) {
        write_or_recover(&self.users).timeout = timeout;
    }

    pub(crate) fn set_users_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.users).api_key = Some(api_key.into());
    }

    pub(crate) fn set_files_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.files).base_url = base_url.into();
    }

    pub(crate) fn set_files_timeout(&self, timeout: Duration) {
        write_or_recover(&self.files).timeout = timeout;
    }

    pub(crate) fn set_files_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.files).api_key = Some(api_key.into());
    }

    pub(crate) fn set_feedback_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.feedback).base_url = base_url.into();
    }

    pub(crate) fn set_feedback_timeout(&self, timeout: Duration) {
        write_or_recover(&self.feedback).timeout = timeout;
    }

    pub(crate) fn set_feedback_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.feedback).api_key = Some(api_key.into());
    }

    pub(crate) fn set_messages_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.messages).base_url = base_url.into();
    }

    pub(crate) fn set_messages_timeout(&self, timeout: Duration) {
        write_or_recover(&self.messages).timeout = timeout;
    }

    pub(crate) fn set_messages_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.messages).api_key = Some(api_key.into());
    }

    pub(crate) fn set_tags_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.tags).base_url = base_url.into();
    }

    pub(crate) fn set_tags_timeout(&self, timeout: Duration) {
        write_or_recover(&self.tags).timeout = timeout;
    }

    pub(crate) fn set_tags_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.tags).api_key = Some(api_key.into());
    }

    pub(crate) fn set_triage_tickets_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.triage_tickets).base_url = base_url.into();
    }

    pub(crate) fn set_triage_tickets_timeout(&self, timeout: Duration) {
        write_or_recover(&self.triage_tickets).timeout = timeout;
    }

    pub(crate) fn set_triage_tickets_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.triage_tickets).api_key = Some(api_key.into());
    }

    pub(crate) fn set_phone_numbers_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.phone_numbers).base_url = base_url.into();
    }

    pub(crate) fn set_phone_numbers_timeout(&self, timeout: Duration) {
        write_or_recover(&self.phone_numbers).timeout = timeout;
    }

    pub(crate) fn set_phone_numbers_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.phone_numbers).api_key = Some(api_key.into());
    }

    pub(crate) fn set_llm_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.llm).base_url = base_url.into();
    }

    pub(crate) fn set_llm_timeout(&self, timeout: Duration) {
        write_or_recover(&self.llm).timeout = timeout;
    }

    pub(crate) fn set_llm_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.llm).api_key = Some(api_key.into());
    }

    pub(crate) fn set_live_count_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.live_count).base_url = base_url.into();
    }

    pub(crate) fn set_live_count_timeout(&self, timeout: Duration) {
        write_or_recover(&self.live_count).timeout = timeout;
    }

    pub(crate) fn set_live_count_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.live_count).api_key = Some(api_key.into());
    }

    pub(crate) fn set_documents_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.documents).base_url = base_url.into();
    }

    pub(crate) fn set_documents_timeout(&self, timeout: Duration) {
        write_or_recover(&self.documents).timeout = timeout;
    }

    pub(crate) fn set_documents_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.documents).api_key = Some(api_key.into());
    }

    pub(crate) fn set_crawl_jobs_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.crawl_jobs).base_url = base_url.into();
    }

    pub(crate) fn set_crawl_jobs_timeout(&self, timeout: Duration) {
        write_or_recover(&self.crawl_jobs).timeout = timeout;
    }

    pub(crate) fn set_crawl_jobs_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.crawl_jobs).api_key = Some(api_key.into());
    }

    pub(crate) fn set_document_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.document).base_url = base_url.into();
    }

    pub(crate) fn set_document_timeout(&self, timeout: Duration) {
        write_or_recover(&self.document).timeout = timeout;
    }

    pub(crate) fn set_document_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.document).api_key = Some(api_key.into());
    }

    pub(crate) fn set_chunk_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.chunk).base_url = base_url.into();
    }

    pub(crate) fn set_chunk_timeout(&self, timeout: Duration) {
        write_or_recover(&self.chunk).timeout = timeout;
    }

    pub(crate) fn set_chunk_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.chunk).api_key = Some(api_key.into());
    }

    pub(crate) fn set_chunks_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.chunks).base_url = base_url.into();
    }

    pub(crate) fn set_chunks_timeout(&self, timeout: Duration) {
        write_or_recover(&self.chunks).timeout = timeout;
    }

    pub(crate) fn set_chunks_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.chunks).api_key = Some(api_key.into());
    }

    pub(crate) fn set_topics_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.topics).base_url = base_url.into();
    }

    pub(crate) fn set_topics_timeout(&self, timeout: Duration) {
        write_or_recover(&self.topics).timeout = timeout;
    }

    pub(crate) fn set_topics_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.topics).api_key = Some(api_key.into());
    }

    pub(crate) fn set_tools_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.tools).base_url = base_url.into();
    }

    pub(crate) fn set_tools_timeout(&self, timeout: Duration) {
        write_or_recover(&self.tools).timeout = timeout;
    }

    pub(crate) fn set_tools_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.tools).api_key = Some(api_key.into());
    }

    pub(crate) fn set_executions_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.executions).base_url = base_url.into();
    }

    pub(crate) fn set_executions_timeout(&self, timeout: Duration) {
        write_or_recover(&self.executions).timeout = timeout;
    }

    pub(crate) fn set_executions_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.executions).api_key = Some(api_key.into());
    }

    pub(crate) fn set_analysis_items_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.analysis_items).base_url = base_url.into();
    }

    pub(crate) fn set_analysis_items_timeout(&self, timeout: Duration) {
        write_or_recover(&self.analysis_items).timeout = timeout;
    }

    pub(crate) fn set_analysis_items_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.analysis_items).api_key = Some(api_key.into());
    }

    pub(crate) fn set_dependent_agents_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.dependent_agents).base_url = base_url.into();
    }

    pub(crate) fn set_dependent_agents_timeout(&self, timeout: Duration) {
        write_or_recover(&self.dependent_agents).timeout = timeout;
    }

    pub(crate) fn set_dependent_agents_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.dependent_agents).api_key = Some(api_key.into());
    }

    pub(crate) fn set_secrets_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.secrets).base_url = base_url.into();
    }

    pub(crate) fn set_secrets_timeout(&self, timeout: Duration) {
        write_or_recover(&self.secrets).timeout = timeout;
    }

    pub(crate) fn set_secrets_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.secrets).api_key = Some(api_key.into());
    }

    pub(crate) fn set_batch_calls_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.batch_calls).base_url = base_url.into();
    }

    pub(crate) fn set_batch_calls_timeout(&self, timeout: Duration) {
        write_or_recover(&self.batch_calls).timeout = timeout;
    }

    pub(crate) fn set_batch_calls_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.batch_calls).api_key = Some(api_key.into());
    }

    pub(crate) fn set_sip_trunk_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.sip_trunk).base_url = base_url.into();
    }

    pub(crate) fn set_sip_trunk_timeout(&self, timeout: Duration) {
        write_or_recover(&self.sip_trunk).timeout = timeout;
    }

    pub(crate) fn set_sip_trunk_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.sip_trunk).api_key = Some(api_key.into());
    }

    pub(crate) fn set_mcp_servers_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.mcp_servers).base_url = base_url.into();
    }

    pub(crate) fn set_mcp_servers_timeout(&self, timeout: Duration) {
        write_or_recover(&self.mcp_servers).timeout = timeout;
    }

    pub(crate) fn set_mcp_servers_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.mcp_servers).api_key = Some(api_key.into());
    }

    pub(crate) fn set_tool_approvals_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.tool_approvals).base_url = base_url.into();
    }

    pub(crate) fn set_tool_approvals_timeout(&self, timeout: Duration) {
        write_or_recover(&self.tool_approvals).timeout = timeout;
    }

    pub(crate) fn set_tool_approvals_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.tool_approvals).api_key = Some(api_key.into());
    }

    pub(crate) fn set_tool_configs_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.tool_configs).base_url = base_url.into();
    }

    pub(crate) fn set_tool_configs_timeout(&self, timeout: Duration) {
        write_or_recover(&self.tool_configs).timeout = timeout;
    }

    pub(crate) fn set_tool_configs_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.tool_configs).api_key = Some(api_key.into());
    }

    pub(crate) fn set_whatsapp_accounts_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.whatsapp_accounts).base_url = base_url.into();
    }

    pub(crate) fn set_whatsapp_accounts_timeout(&self, timeout: Duration) {
        write_or_recover(&self.whatsapp_accounts).timeout = timeout;
    }

    pub(crate) fn set_whatsapp_accounts_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.whatsapp_accounts).api_key = Some(api_key.into());
    }

    pub(crate) fn set_branches_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.branches).base_url = base_url.into();
    }

    pub(crate) fn set_branches_timeout(&self, timeout: Duration) {
        write_or_recover(&self.branches).timeout = timeout;
    }

    pub(crate) fn set_branches_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.branches).api_key = Some(api_key.into());
    }

    pub(crate) fn set_versions_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.versions).base_url = base_url.into();
    }

    pub(crate) fn set_versions_timeout(&self, timeout: Duration) {
        write_or_recover(&self.versions).timeout = timeout;
    }

    pub(crate) fn set_versions_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.versions).api_key = Some(api_key.into());
    }

    pub(crate) fn set_deployments_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.deployments).base_url = base_url.into();
    }

    pub(crate) fn set_deployments_timeout(&self, timeout: Duration) {
        write_or_recover(&self.deployments).timeout = timeout;
    }

    pub(crate) fn set_deployments_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.deployments).api_key = Some(api_key.into());
    }

    pub(crate) fn set_drafts_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.drafts).base_url = base_url.into();
    }

    pub(crate) fn set_drafts_timeout(&self, timeout: Duration) {
        write_or_recover(&self.drafts).timeout = timeout;
    }

    pub(crate) fn set_drafts_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.drafts).api_key = Some(api_key.into());
    }

    pub(crate) fn set_merge_proposals_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.merge_proposals).base_url = base_url.into();
    }

    pub(crate) fn set_merge_proposals_timeout(&self, timeout: Duration) {
        write_or_recover(&self.merge_proposals).timeout = timeout;
    }

    pub(crate) fn set_merge_proposals_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.merge_proposals).api_key = Some(api_key.into());
    }

    pub(crate) fn set_config_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.config).base_url = base_url.into();
    }

    pub(crate) fn set_config_timeout(&self, timeout: Duration) {
        write_or_recover(&self.config).timeout = timeout;
    }

    pub(crate) fn set_config_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.config).api_key = Some(api_key.into());
    }

    pub(crate) fn set_outbound_campaigns_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.outbound_campaigns).base_url = base_url.into();
    }

    pub(crate) fn set_outbound_campaigns_timeout(&self, timeout: Duration) {
        write_or_recover(&self.outbound_campaigns).timeout = timeout;
    }

    pub(crate) fn set_outbound_campaigns_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.outbound_campaigns).api_key = Some(api_key.into());
    }

    pub(crate) fn set_leads_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.leads).base_url = base_url.into();
    }

    pub(crate) fn set_leads_timeout(&self, timeout: Duration) {
        write_or_recover(&self.leads).timeout = timeout;
    }

    pub(crate) fn set_leads_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.leads).api_key = Some(api_key.into());
    }

    pub(crate) fn set_calls_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.calls).base_url = base_url.into();
    }

    pub(crate) fn set_calls_timeout(&self, timeout: Duration) {
        write_or_recover(&self.calls).timeout = timeout;
    }

    pub(crate) fn set_calls_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.calls).api_key = Some(api_key.into());
    }

    pub(crate) fn set_attention_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.attention).base_url = base_url.into();
    }

    pub(crate) fn set_attention_timeout(&self, timeout: Duration) {
        write_or_recover(&self.attention).timeout = timeout;
    }

    pub(crate) fn set_attention_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.attention).api_key = Some(api_key.into());
    }

    pub(crate) fn set_speech_engine_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.speech_engine).base_url = base_url.into();
    }

    pub(crate) fn set_speech_engine_timeout(&self, timeout: Duration) {
        write_or_recover(&self.speech_engine).timeout = timeout;
    }

    pub(crate) fn set_speech_engine_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.speech_engine).api_key = Some(api_key.into());
    }

    pub(crate) fn set_procedures_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.procedures).base_url = base_url.into();
    }

    pub(crate) fn set_procedures_timeout(&self, timeout: Duration) {
        write_or_recover(&self.procedures).timeout = timeout;
    }

    pub(crate) fn set_procedures_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.procedures).api_key = Some(api_key.into());
    }

    pub(crate) fn set_analysis_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.analysis).base_url = base_url.into();
    }

    pub(crate) fn set_analysis_timeout(&self, timeout: Duration) {
        write_or_recover(&self.analysis).timeout = timeout;
    }

    pub(crate) fn set_analysis_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.analysis).api_key = Some(api_key.into());
    }

    pub(crate) fn set_environment_variables_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.environment_variables).base_url = base_url.into();
    }

    pub(crate) fn set_environment_variables_timeout(&self, timeout: Duration) {
        write_or_recover(&self.environment_variables).timeout = timeout;
    }

    pub(crate) fn set_environment_variables_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.environment_variables).api_key = Some(api_key.into());
    }

    pub(crate) fn set_environments_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.environments).base_url = base_url.into();
    }

    pub(crate) fn set_environments_timeout(&self, timeout: Duration) {
        write_or_recover(&self.environments).timeout = timeout;
    }

    pub(crate) fn set_environments_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.environments).api_key = Some(api_key.into());
    }

    pub(crate) fn set_realtime_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.realtime).base_url = base_url.into();
    }

    pub(crate) fn set_realtime_timeout(&self, timeout: Duration) {
        write_or_recover(&self.realtime).timeout = timeout;
    }

    pub(crate) fn set_realtime_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.realtime).api_key = Some(api_key.into());
    }

    pub(crate) fn set_composition_plan_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.composition_plan).base_url = base_url.into();
    }

    pub(crate) fn set_composition_plan_timeout(&self, timeout: Duration) {
        write_or_recover(&self.composition_plan).timeout = timeout;
    }

    pub(crate) fn set_composition_plan_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.composition_plan).api_key = Some(api_key.into());
    }

    pub(crate) fn set_finetunes_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.finetunes).base_url = base_url.into();
    }

    pub(crate) fn set_finetunes_timeout(&self, timeout: Duration) {
        write_or_recover(&self.finetunes).timeout = timeout;
    }

    pub(crate) fn set_finetunes_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.finetunes).api_key = Some(api_key.into());
    }

    pub(crate) fn set_orders_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.orders).base_url = base_url.into();
    }

    pub(crate) fn set_orders_timeout(&self, timeout: Duration) {
        write_or_recover(&self.orders).timeout = timeout;
    }

    pub(crate) fn set_orders_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.orders).api_key = Some(api_key.into());
    }

    pub(crate) fn set_media_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.media).base_url = base_url.into();
    }

    pub(crate) fn set_media_timeout(&self, timeout: Duration) {
        write_or_recover(&self.media).timeout = timeout;
    }

    pub(crate) fn set_media_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.media).api_key = Some(api_key.into());
    }

    pub(crate) fn set_items_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.items).base_url = base_url.into();
    }

    pub(crate) fn set_items_timeout(&self, timeout: Duration) {
        write_or_recover(&self.items).timeout = timeout;
    }

    pub(crate) fn set_items_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.items).api_key = Some(api_key.into());
    }

    pub(crate) fn set_deliverables_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.deliverables).base_url = base_url.into();
    }

    pub(crate) fn set_deliverables_timeout(&self, timeout: Duration) {
        write_or_recover(&self.deliverables).timeout = timeout;
    }

    pub(crate) fn set_deliverables_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.deliverables).api_key = Some(api_key.into());
    }

    pub(crate) fn set_languages_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.languages).base_url = base_url.into();
    }

    pub(crate) fn set_languages_timeout(&self, timeout: Duration) {
        write_or_recover(&self.languages).timeout = timeout;
    }

    pub(crate) fn set_languages_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.languages).api_key = Some(api_key.into());
    }

    pub(crate) fn set_pvc_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.pvc).base_url = base_url.into();
    }

    pub(crate) fn set_pvc_timeout(&self, timeout: Duration) {
        write_or_recover(&self.pvc).timeout = timeout;
    }

    pub(crate) fn set_pvc_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.pvc).api_key = Some(api_key.into());
    }

    pub(crate) fn set_waveform_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.waveform).base_url = base_url.into();
    }

    pub(crate) fn set_waveform_timeout(&self, timeout: Duration) {
        write_or_recover(&self.waveform).timeout = timeout;
    }

    pub(crate) fn set_waveform_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.waveform).api_key = Some(api_key.into());
    }

    pub(crate) fn set_speakers_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.speakers).base_url = base_url.into();
    }

    pub(crate) fn set_speakers_timeout(&self, timeout: Duration) {
        write_or_recover(&self.speakers).timeout = timeout;
    }

    pub(crate) fn set_speakers_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.speakers).api_key = Some(api_key.into());
    }

    pub(crate) fn set_captcha_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.captcha).base_url = base_url.into();
    }

    pub(crate) fn set_captcha_timeout(&self, timeout: Duration) {
        write_or_recover(&self.captcha).timeout = timeout;
    }

    pub(crate) fn set_captcha_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.captcha).api_key = Some(api_key.into());
    }

    pub(crate) fn set_verification_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.verification).base_url = base_url.into();
    }

    pub(crate) fn set_verification_timeout(&self, timeout: Duration) {
        write_or_recover(&self.verification).timeout = timeout;
    }

    pub(crate) fn set_verification_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.verification).api_key = Some(api_key.into());
    }

    pub(crate) fn set_video_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.video).base_url = base_url.into();
    }

    pub(crate) fn set_video_timeout(&self, timeout: Duration) {
        write_or_recover(&self.video).timeout = timeout;
    }

    pub(crate) fn set_video_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.video).api_key = Some(api_key.into());
    }

    pub(crate) fn set_image_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.image).base_url = base_url.into();
    }

    pub(crate) fn set_image_timeout(&self, timeout: Duration) {
        write_or_recover(&self.image).timeout = timeout;
    }

    pub(crate) fn set_image_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.image).api_key = Some(api_key.into());
    }

    pub(crate) fn set_runs_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.runs).base_url = base_url.into();
    }

    pub(crate) fn set_runs_timeout(&self, timeout: Duration) {
        write_or_recover(&self.runs).timeout = timeout;
    }

    pub(crate) fn set_runs_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.runs).api_key = Some(api_key.into());
    }

    pub(crate) fn set_templates_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.templates).base_url = base_url.into();
    }

    pub(crate) fn set_templates_timeout(&self, timeout: Duration) {
        write_or_recover(&self.templates).timeout = timeout;
    }

    pub(crate) fn set_templates_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.templates).api_key = Some(api_key.into());
    }

    pub(crate) fn set_assets_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.assets).base_url = base_url.into();
    }

    pub(crate) fn set_assets_timeout(&self, timeout: Duration) {
        write_or_recover(&self.assets).timeout = timeout;
    }

    pub(crate) fn set_assets_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.assets).api_key = Some(api_key.into());
    }

    pub(crate) fn set_usage_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.usage).base_url = base_url.into();
    }

    pub(crate) fn set_usage_timeout(&self, timeout: Duration) {
        write_or_recover(&self.usage).timeout = timeout;
    }

    pub(crate) fn set_usage_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.usage).api_key = Some(api_key.into());
    }

    pub(crate) fn set_requests_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.requests).base_url = base_url.into();
    }

    pub(crate) fn set_requests_timeout(&self, timeout: Duration) {
        write_or_recover(&self.requests).timeout = timeout;
    }

    pub(crate) fn set_requests_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.requests).api_key = Some(api_key.into());
    }

    pub(crate) fn set_services_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.services).base_url = base_url.into();
    }

    pub(crate) fn set_services_timeout(&self, timeout: Duration) {
        write_or_recover(&self.services).timeout = timeout;
    }

    pub(crate) fn set_services_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.services).api_key = Some(api_key.into());
    }

    pub(crate) fn set_products_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.products).base_url = base_url.into();
    }

    pub(crate) fn set_products_timeout(&self, timeout: Duration) {
        write_or_recover(&self.products).timeout = timeout;
    }

    pub(crate) fn set_products_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.products).api_key = Some(api_key.into());
    }

    pub(crate) fn set_stock_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.stock).base_url = base_url.into();
    }

    pub(crate) fn set_stock_timeout(&self, timeout: Duration) {
        write_or_recover(&self.stock).timeout = timeout;
    }

    pub(crate) fn set_stock_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.stock).api_key = Some(api_key.into());
    }

    pub(crate) fn set_staff_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.staff).base_url = base_url.into();
    }

    pub(crate) fn set_staff_timeout(&self, timeout: Duration) {
        write_or_recover(&self.staff).timeout = timeout;
    }

    pub(crate) fn set_staff_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.staff).api_key = Some(api_key.into());
    }

    pub(crate) fn set_receptionists_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.receptionists).base_url = base_url.into();
    }

    pub(crate) fn set_receptionists_timeout(&self, timeout: Duration) {
        write_or_recover(&self.receptionists).timeout = timeout;
    }

    pub(crate) fn set_receptionists_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.receptionists).api_key = Some(api_key.into());
    }

    pub(crate) fn set_transfer_rules_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.transfer_rules).base_url = base_url.into();
    }

    pub(crate) fn set_transfer_rules_timeout(&self, timeout: Duration) {
        write_or_recover(&self.transfer_rules).timeout = timeout;
    }

    pub(crate) fn set_transfer_rules_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.transfer_rules).api_key = Some(api_key.into());
    }

    pub(crate) fn set_holidays_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.holidays).base_url = base_url.into();
    }

    pub(crate) fn set_holidays_timeout(&self, timeout: Duration) {
        write_or_recover(&self.holidays).timeout = timeout;
    }

    pub(crate) fn set_holidays_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.holidays).api_key = Some(api_key.into());
    }

    pub(crate) fn set_locations_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.locations).base_url = base_url.into();
    }

    pub(crate) fn set_locations_timeout(&self, timeout: Duration) {
        write_or_recover(&self.locations).timeout = timeout;
    }

    pub(crate) fn set_locations_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.locations).api_key = Some(api_key.into());
    }

    pub(crate) fn set_business_profile_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.business_profile).base_url = base_url.into();
    }

    pub(crate) fn set_business_profile_timeout(&self, timeout: Duration) {
        write_or_recover(&self.business_profile).timeout = timeout;
    }

    pub(crate) fn set_business_profile_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.business_profile).api_key = Some(api_key.into());
    }

    pub(crate) fn set_knowledge_sources_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.knowledge_sources).base_url = base_url.into();
    }

    pub(crate) fn set_knowledge_sources_timeout(&self, timeout: Duration) {
        write_or_recover(&self.knowledge_sources).timeout = timeout;
    }

    pub(crate) fn set_knowledge_sources_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.knowledge_sources).api_key = Some(api_key.into());
    }

    pub(crate) fn set_workspaces_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.workspaces).base_url = base_url.into();
    }

    pub(crate) fn set_workspaces_timeout(&self, timeout: Duration) {
        write_or_recover(&self.workspaces).timeout = timeout;
    }

    pub(crate) fn set_workspaces_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.workspaces).api_key = Some(api_key.into());
    }

    pub(crate) fn set_subscriptions_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.subscriptions).base_url = base_url.into();
    }

    pub(crate) fn set_subscriptions_timeout(&self, timeout: Duration) {
        write_or_recover(&self.subscriptions).timeout = timeout;
    }

    pub(crate) fn set_subscriptions_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.subscriptions).api_key = Some(api_key.into());
    }

    pub(crate) fn set_tiers_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.tiers).base_url = base_url.into();
    }

    pub(crate) fn set_tiers_timeout(&self, timeout: Duration) {
        write_or_recover(&self.tiers).timeout = timeout;
    }

    pub(crate) fn set_tiers_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.tiers).api_key = Some(api_key.into());
    }

    pub(crate) fn set_billing_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.billing).base_url = base_url.into();
    }

    pub(crate) fn set_billing_timeout(&self, timeout: Duration) {
        write_or_recover(&self.billing).timeout = timeout;
    }

    pub(crate) fn set_billing_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.billing).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_text_to_dialogue_stream_input_base_url(
        &self,
        base_url: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_dialogue_stream_input).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_text_to_dialogue_stream_input_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_text_to_dialogue_stream_input).timeout = timeout;
    }

    pub(crate) fn set_v_1_text_to_dialogue_stream_input_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.v_1_text_to_dialogue_stream_input).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_text_to_dialogue_multi_stream_input_base_url(
        &self,
        base_url: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_text_to_dialogue_multi_stream_input_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).timeout = timeout;
    }

    pub(crate) fn set_v_1_text_to_dialogue_multi_stream_input_api_key(
        &self,
        api_key: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_dialogue_multi_stream_input).api_key =
            Some(api_key.into());
    }

    pub(crate) fn set_speech_engine_upstream_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.speech_engine_upstream).base_url = base_url.into();
    }

    pub(crate) fn set_speech_engine_upstream_timeout(&self, timeout: Duration) {
        write_or_recover(&self.speech_engine_upstream).timeout = timeout;
    }

    pub(crate) fn set_speech_engine_upstream_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.speech_engine_upstream).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_speech_to_text_realtime_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.v_1_speech_to_text_realtime).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_speech_to_text_realtime_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_speech_to_text_realtime).timeout = timeout;
    }

    pub(crate) fn set_v_1_speech_to_text_realtime_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.v_1_speech_to_text_realtime).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_text_to_speech_voice_id_stream_input_base_url(
        &self,
        base_url: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_text_to_speech_voice_id_stream_input_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).timeout = timeout;
    }

    pub(crate) fn set_v_1_text_to_speech_voice_id_stream_input_api_key(
        &self,
        api_key: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_speech_voice_id_stream_input).api_key =
            Some(api_key.into());
    }

    pub(crate) fn set_v_1_text_to_speech_voice_id_multi_stream_input_base_url(
        &self,
        base_url: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).base_url =
            base_url.into();
    }

    pub(crate) fn set_v_1_text_to_speech_voice_id_multi_stream_input_timeout(
        &self,
        timeout: Duration,
    ) {
        write_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).timeout = timeout;
    }

    pub(crate) fn set_v_1_text_to_speech_voice_id_multi_stream_input_api_key(
        &self,
        api_key: impl Into<String>,
    ) {
        write_or_recover(&self.v_1_text_to_speech_voice_id_multi_stream_input).api_key =
            Some(api_key.into());
    }

    pub(crate) fn set_v_1_translation_realtime_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.v_1_translation_realtime).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_translation_realtime_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_translation_realtime).timeout = timeout;
    }

    pub(crate) fn set_v_1_translation_realtime_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.v_1_translation_realtime).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_dubbing_live_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.v_1_dubbing_live).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_dubbing_live_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_dubbing_live).timeout = timeout;
    }

    pub(crate) fn set_v_1_dubbing_live_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.v_1_dubbing_live).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_subtitling_realtime_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.v_1_subtitling_realtime).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_subtitling_realtime_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_subtitling_realtime).timeout = timeout;
    }

    pub(crate) fn set_v_1_subtitling_realtime_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.v_1_subtitling_realtime).api_key = Some(api_key.into());
    }

    pub(crate) fn set_v_1_convai_conversation_base_url(&self, base_url: impl Into<String>) {
        write_or_recover(&self.v_1_convai_conversation).base_url = base_url.into();
    }

    pub(crate) fn set_v_1_convai_conversation_timeout(&self, timeout: Duration) {
        write_or_recover(&self.v_1_convai_conversation).timeout = timeout;
    }

    pub(crate) fn set_v_1_convai_conversation_api_key(&self, api_key: impl Into<String>) {
        write_or_recover(&self.v_1_convai_conversation).api_key = Some(api_key.into());
    }
}
