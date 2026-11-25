use elevenlabs_rust_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig::default();
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .history
        .list(
            &ListQueryRequest {
                page_size: Some(100),
                start_after_history_item_id: Some("start_after_history_item_id".to_string()),
                voice_id: Some("voice_id".to_string()),
                model_id: Some("model_id".to_string()),
                date_before_unix: Some(8),
                date_after_unix: Some(2),
                sort_direction: Some(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort::Asc),
                search: Some("search".to_string()),
                source: Some(ListSource::Tts),
                sources: Some(vec![ListSource::Tts]),
                ..Default::default()
            },
            None,
        )
        .await;
}
