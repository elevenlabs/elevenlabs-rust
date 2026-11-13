pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectVideoResponseModel {
    #[serde(default)]
    pub video_id: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub signed_url: Option<String>,
    #[serde(default)]
    pub signed_preview_url: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub offset_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub duration_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub volume_gain_db: f64,
    #[serde(default)]
    pub muted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub fade_in_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub fade_out_ms: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub width: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub height: i64,
    #[serde(default)]
    pub codec: String,
    #[serde(default)]
    pub order: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub updated_at_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float")]
    pub thumbnail_interval_seconds: f64,
    #[serde(default)]
    pub thumbnail_size: Vec<i64>,
    #[serde(default)]
    pub thumbnail_sheets: Vec<ProjectVideoThumbnailSheetResponseModel>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub start_time_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub end_time_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_preview_signed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_video_id: Option<String>,
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
    pub audio_track_ready: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format_ready: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_snapshot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_context: Option<GenerationSourceContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<VideoAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<AssetTranscription>,
    /// Defines asset positioning and transformation on canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_placement: Option<CanvasPlacement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<ClipAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub playback_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub preview_job_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub import_speech_progress: Option<f64>,
}

impl ProjectVideoResponseModel {
    pub fn builder() -> ProjectVideoResponseModelBuilder {
        <ProjectVideoResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectVideoResponseModelBuilder {
    video_id: Option<String>,
    filename: Option<String>,
    signed_url: Option<String>,
    signed_preview_url: Option<String>,
    offset_ms: Option<i64>,
    duration_ms: Option<i64>,
    volume_gain_db: Option<f64>,
    muted: Option<bool>,
    fade_in_ms: Option<i64>,
    fade_out_ms: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    codec: Option<String>,
    order: Option<String>,
    created_at_ms: Option<i64>,
    updated_at_ms: Option<i64>,
    error: Option<String>,
    thumbnail_interval_seconds: Option<f64>,
    thumbnail_size: Option<Vec<i64>>,
    thumbnail_sheets: Option<Vec<ProjectVideoThumbnailSheetResponseModel>>,
    start_time_ms: Option<i64>,
    end_time_ms: Option<i64>,
    asset_preview_signed_url: Option<String>,
    source_video_id: Option<String>,
    source_asset_id: Option<String>,
    source_platform_asset_id: Option<String>,
    pending_blocks_metadata: Option<PendingBlocksMetadataModel>,
    pending_external_audios_metadata: Option<PendingExternalAudiosMetadataModel>,
    speech_imported: Option<bool>,
    pending_task: Option<PendingClipTask>,
    audio_track_ready: Option<bool>,
    export_format_ready: Option<bool>,
    current_snapshot_id: Option<String>,
    source_context: Option<GenerationSourceContext>,
    analysis: Option<VideoAnalysis>,
    transcription: Option<AssetTranscription>,
    canvas_placement: Option<CanvasPlacement>,
    animation: Option<ClipAnimation>,
    playback_speed: Option<f64>,
    opacity: Option<f64>,
    track_id: Option<String>,
    preview_job_progress: Option<f64>,
    import_speech_progress: Option<f64>,
}

impl ProjectVideoResponseModelBuilder {
    pub fn video_id(mut self, value: impl Into<String>) -> Self {
        self.video_id = Some(value.into());
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

    pub fn signed_preview_url(mut self, value: impl Into<String>) -> Self {
        self.signed_preview_url = Some(value.into());
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

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn codec(mut self, value: impl Into<String>) -> Self {
        self.codec = Some(value.into());
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
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

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn thumbnail_interval_seconds(mut self, value: f64) -> Self {
        self.thumbnail_interval_seconds = Some(value);
        self
    }

    pub fn thumbnail_size(mut self, value: Vec<i64>) -> Self {
        self.thumbnail_size = Some(value);
        self
    }

    pub fn thumbnail_sheets(mut self, value: Vec<ProjectVideoThumbnailSheetResponseModel>) -> Self {
        self.thumbnail_sheets = Some(value);
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

    pub fn asset_preview_signed_url(mut self, value: impl Into<String>) -> Self {
        self.asset_preview_signed_url = Some(value.into());
        self
    }

    pub fn source_video_id(mut self, value: impl Into<String>) -> Self {
        self.source_video_id = Some(value.into());
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

    pub fn audio_track_ready(mut self, value: bool) -> Self {
        self.audio_track_ready = Some(value);
        self
    }

    pub fn export_format_ready(mut self, value: bool) -> Self {
        self.export_format_ready = Some(value);
        self
    }

    pub fn current_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.current_snapshot_id = Some(value.into());
        self
    }

    pub fn source_context(mut self, value: GenerationSourceContext) -> Self {
        self.source_context = Some(value);
        self
    }

    pub fn analysis(mut self, value: VideoAnalysis) -> Self {
        self.analysis = Some(value);
        self
    }

    pub fn transcription(mut self, value: AssetTranscription) -> Self {
        self.transcription = Some(value);
        self
    }

    pub fn canvas_placement(mut self, value: CanvasPlacement) -> Self {
        self.canvas_placement = Some(value);
        self
    }

    pub fn animation(mut self, value: ClipAnimation) -> Self {
        self.animation = Some(value);
        self
    }

    pub fn playback_speed(mut self, value: f64) -> Self {
        self.playback_speed = Some(value);
        self
    }

    pub fn opacity(mut self, value: f64) -> Self {
        self.opacity = Some(value);
        self
    }

    pub fn track_id(mut self, value: impl Into<String>) -> Self {
        self.track_id = Some(value.into());
        self
    }

    pub fn preview_job_progress(mut self, value: f64) -> Self {
        self.preview_job_progress = Some(value);
        self
    }

    pub fn import_speech_progress(mut self, value: f64) -> Self {
        self.import_speech_progress = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectVideoResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`video_id`](ProjectVideoResponseModelBuilder::video_id)
    /// - [`filename`](ProjectVideoResponseModelBuilder::filename)
    /// - [`offset_ms`](ProjectVideoResponseModelBuilder::offset_ms)
    /// - [`duration_ms`](ProjectVideoResponseModelBuilder::duration_ms)
    /// - [`volume_gain_db`](ProjectVideoResponseModelBuilder::volume_gain_db)
    /// - [`muted`](ProjectVideoResponseModelBuilder::muted)
    /// - [`width`](ProjectVideoResponseModelBuilder::width)
    /// - [`height`](ProjectVideoResponseModelBuilder::height)
    /// - [`codec`](ProjectVideoResponseModelBuilder::codec)
    /// - [`order`](ProjectVideoResponseModelBuilder::order)
    /// - [`created_at_ms`](ProjectVideoResponseModelBuilder::created_at_ms)
    /// - [`updated_at_ms`](ProjectVideoResponseModelBuilder::updated_at_ms)
    /// - [`thumbnail_interval_seconds`](ProjectVideoResponseModelBuilder::thumbnail_interval_seconds)
    /// - [`thumbnail_size`](ProjectVideoResponseModelBuilder::thumbnail_size)
    /// - [`thumbnail_sheets`](ProjectVideoResponseModelBuilder::thumbnail_sheets)
    /// - [`start_time_ms`](ProjectVideoResponseModelBuilder::start_time_ms)
    pub fn build(self) -> Result<ProjectVideoResponseModel, BuildError> {
        Ok(ProjectVideoResponseModel {
            video_id: self
                .video_id
                .ok_or_else(|| BuildError::missing_field("video_id"))?,
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            signed_url: self.signed_url,
            signed_preview_url: self.signed_preview_url,
            offset_ms: self
                .offset_ms
                .ok_or_else(|| BuildError::missing_field("offset_ms"))?,
            duration_ms: self
                .duration_ms
                .ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            volume_gain_db: self
                .volume_gain_db
                .ok_or_else(|| BuildError::missing_field("volume_gain_db"))?,
            muted: self
                .muted
                .ok_or_else(|| BuildError::missing_field("muted"))?,
            fade_in_ms: self.fade_in_ms,
            fade_out_ms: self.fade_out_ms,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            codec: self
                .codec
                .ok_or_else(|| BuildError::missing_field("codec"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            created_at_ms: self
                .created_at_ms
                .ok_or_else(|| BuildError::missing_field("created_at_ms"))?,
            updated_at_ms: self
                .updated_at_ms
                .ok_or_else(|| BuildError::missing_field("updated_at_ms"))?,
            error: self.error,
            thumbnail_interval_seconds: self
                .thumbnail_interval_seconds
                .ok_or_else(|| BuildError::missing_field("thumbnail_interval_seconds"))?,
            thumbnail_size: self
                .thumbnail_size
                .ok_or_else(|| BuildError::missing_field("thumbnail_size"))?,
            thumbnail_sheets: self
                .thumbnail_sheets
                .ok_or_else(|| BuildError::missing_field("thumbnail_sheets"))?,
            start_time_ms: self
                .start_time_ms
                .ok_or_else(|| BuildError::missing_field("start_time_ms"))?,
            end_time_ms: self.end_time_ms,
            asset_preview_signed_url: self.asset_preview_signed_url,
            source_video_id: self.source_video_id,
            source_asset_id: self.source_asset_id,
            source_platform_asset_id: self.source_platform_asset_id,
            pending_blocks_metadata: self.pending_blocks_metadata,
            pending_external_audios_metadata: self.pending_external_audios_metadata,
            speech_imported: self.speech_imported,
            pending_task: self.pending_task,
            audio_track_ready: self.audio_track_ready,
            export_format_ready: self.export_format_ready,
            current_snapshot_id: self.current_snapshot_id,
            source_context: self.source_context,
            analysis: self.analysis,
            transcription: self.transcription,
            canvas_placement: self.canvas_placement,
            animation: self.animation,
            playback_speed: self.playback_speed,
            opacity: self.opacity,
            track_id: self.track_id,
            preview_job_progress: self.preview_job_progress,
            import_speech_progress: self.import_speech_progress,
        })
    }
}
