pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectExternalAudioResponseModel {
    #[serde(default)]
    pub external_audio_id: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub signed_url: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub offset_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub duration_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub start_time_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub end_time_ms: Option<i64>,
    #[serde(default)]
    pub order: String,
    #[serde(default)]
    pub track_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub updated_at_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub volume_gain_db: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub fade_in_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub fade_out_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_external_audio_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_platform_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_blocks_metadata: Option<PendingBlocksMetadataModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_external_audios_metadata: Option<PendingExternalAudiosMetadataModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_imported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_task: Option<PendingClipTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_snapshot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_context: Option<SourceContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<AudioAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<AssetTranscription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub import_speech_progress: Option<f64>,
}

impl ProjectExternalAudioResponseModel {
    pub fn builder() -> ProjectExternalAudioResponseModelBuilder {
        <ProjectExternalAudioResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectExternalAudioResponseModelBuilder {
    external_audio_id: Option<String>,
    filename: Option<String>,
    signed_url: Option<String>,
    offset_ms: Option<i64>,
    duration_ms: Option<i64>,
    start_time_ms: Option<i64>,
    end_time_ms: Option<i64>,
    order: Option<String>,
    track_id: Option<String>,
    created_at_ms: Option<i64>,
    updated_at_ms: Option<i64>,
    volume_gain_db: Option<f64>,
    muted: Option<bool>,
    fade_in_ms: Option<i64>,
    fade_out_ms: Option<i64>,
    source_external_audio_id: Option<String>,
    source_asset_id: Option<String>,
    source_platform_asset_id: Option<String>,
    pending_blocks_metadata: Option<PendingBlocksMetadataModel>,
    pending_external_audios_metadata: Option<PendingExternalAudiosMetadataModel>,
    speech_imported: Option<bool>,
    pending_task: Option<PendingClipTask>,
    error: Option<String>,
    current_snapshot_id: Option<String>,
    source_context: Option<SourceContext>,
    analysis: Option<AudioAnalysis>,
    transcription: Option<AssetTranscription>,
    import_speech_progress: Option<f64>,
}

impl ProjectExternalAudioResponseModelBuilder {
    pub fn external_audio_id(mut self, value: impl Into<String>) -> Self {
        self.external_audio_id = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn signed_url(mut self, value: impl Into<String>) -> Self {
        self.signed_url = Some(value.into());
        self
    }

    pub fn offset_ms(mut self, value: i64) -> Self {
        self.offset_ms = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn start_time_ms(mut self, value: i64) -> Self {
        self.start_time_ms = Some(value);
        self
    }

    pub fn end_time_ms(mut self, value: i64) -> Self {
        self.end_time_ms = Some(value);
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn track_id(mut self, value: impl Into<String>) -> Self {
        self.track_id = Some(value.into());
        self
    }

    pub fn created_at_ms(mut self, value: i64) -> Self {
        self.created_at_ms = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    pub fn volume_gain_db(mut self, value: f64) -> Self {
        self.volume_gain_db = Some(value);
        self
    }

    pub fn muted(mut self, value: bool) -> Self {
        self.muted = Some(value);
        self
    }

    pub fn fade_in_ms(mut self, value: i64) -> Self {
        self.fade_in_ms = Some(value);
        self
    }

    pub fn fade_out_ms(mut self, value: i64) -> Self {
        self.fade_out_ms = Some(value);
        self
    }

    pub fn source_external_audio_id(mut self, value: impl Into<String>) -> Self {
        self.source_external_audio_id = Some(value.into());
        self
    }

    pub fn source_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_asset_id = Some(value.into());
        self
    }

    pub fn source_platform_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_platform_asset_id = Some(value.into());
        self
    }

    pub fn pending_blocks_metadata(mut self, value: PendingBlocksMetadataModel) -> Self {
        self.pending_blocks_metadata = Some(value);
        self
    }

    pub fn pending_external_audios_metadata(
        mut self,
        value: PendingExternalAudiosMetadataModel,
    ) -> Self {
        self.pending_external_audios_metadata = Some(value);
        self
    }

    pub fn speech_imported(mut self, value: bool) -> Self {
        self.speech_imported = Some(value);
        self
    }

    pub fn pending_task(mut self, value: PendingClipTask) -> Self {
        self.pending_task = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn current_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.current_snapshot_id = Some(value.into());
        self
    }

    pub fn source_context(mut self, value: SourceContext) -> Self {
        self.source_context = Some(value);
        self
    }

    pub fn analysis(mut self, value: AudioAnalysis) -> Self {
        self.analysis = Some(value);
        self
    }

    pub fn transcription(mut self, value: AssetTranscription) -> Self {
        self.transcription = Some(value);
        self
    }

    pub fn import_speech_progress(mut self, value: f64) -> Self {
        self.import_speech_progress = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectExternalAudioResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_audio_id`](ProjectExternalAudioResponseModelBuilder::external_audio_id)
    /// - [`filename`](ProjectExternalAudioResponseModelBuilder::filename)
    /// - [`offset_ms`](ProjectExternalAudioResponseModelBuilder::offset_ms)
    /// - [`duration_ms`](ProjectExternalAudioResponseModelBuilder::duration_ms)
    /// - [`start_time_ms`](ProjectExternalAudioResponseModelBuilder::start_time_ms)
    /// - [`order`](ProjectExternalAudioResponseModelBuilder::order)
    /// - [`track_id`](ProjectExternalAudioResponseModelBuilder::track_id)
    /// - [`created_at_ms`](ProjectExternalAudioResponseModelBuilder::created_at_ms)
    /// - [`updated_at_ms`](ProjectExternalAudioResponseModelBuilder::updated_at_ms)
    pub fn build(self) -> Result<ProjectExternalAudioResponseModel, BuildError> {
        Ok(ProjectExternalAudioResponseModel {
            external_audio_id: self
                .external_audio_id
                .ok_or_else(|| BuildError::missing_field("external_audio_id"))?,
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            signed_url: self.signed_url,
            offset_ms: self
                .offset_ms
                .ok_or_else(|| BuildError::missing_field("offset_ms"))?,
            duration_ms: self
                .duration_ms
                .ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            start_time_ms: self
                .start_time_ms
                .ok_or_else(|| BuildError::missing_field("start_time_ms"))?,
            end_time_ms: self.end_time_ms,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            track_id: self
                .track_id
                .ok_or_else(|| BuildError::missing_field("track_id"))?,
            created_at_ms: self
                .created_at_ms
                .ok_or_else(|| BuildError::missing_field("created_at_ms"))?,
            updated_at_ms: self
                .updated_at_ms
                .ok_or_else(|| BuildError::missing_field("updated_at_ms"))?,
            volume_gain_db: self.volume_gain_db,
            muted: self.muted,
            fade_in_ms: self.fade_in_ms,
            fade_out_ms: self.fade_out_ms,
            source_external_audio_id: self.source_external_audio_id,
            source_asset_id: self.source_asset_id,
            source_platform_asset_id: self.source_platform_asset_id,
            pending_blocks_metadata: self.pending_blocks_metadata,
            pending_external_audios_metadata: self.pending_external_audios_metadata,
            speech_imported: self.speech_imported,
            pending_task: self.pending_task,
            error: self.error,
            current_snapshot_id: self.current_snapshot_id,
            source_context: self.source_context,
            analysis: self.analysis,
            transcription: self.transcription,
            import_speech_progress: self.import_speech_progress,
        })
    }
}
