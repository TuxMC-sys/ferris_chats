use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use ferris_chats_data::{AppState, IncomingMessage, Messages};

pub async fn all_messages(State(messages): State<AppState>) -> Json<Messages> {
    Json(messages.data.lock().unwrap().clone())
}
pub async fn get_messages(
    State(messages): State<AppState>,
    Path((first, end)): Path<(usize, usize)>,
) -> Result<Json<Messages>, StatusCode> {
    <Messages as Clone>::clone(&messages.data.lock().unwrap())
        .get_range(first, end)
        .map_or(Err(StatusCode::BAD_REQUEST), |res| Ok(Json(res)))
}
pub async fn message_count(State(messages): State<AppState>) -> String {
    messages.data.lock().unwrap().message_count().to_string()
}
pub async fn messages_from_time(
    State(messages): State<AppState>,
    Path(time): Path<String>,
) -> Result<Json<Messages>, StatusCode> {
    let messages = messages.data.lock().unwrap().clone();
    let Some(index) = messages.last_index_at_time(
        time.parse::<DateTime<Utc>>()
            .map_err(|_| StatusCode::BAD_REQUEST)?,
    ) else {
        return Err(StatusCode::NOT_FOUND);
    };
    messages
        .clone()
        .get_range(index, messages.message_count() + 1)
        .map_or(Err(StatusCode::NOT_FOUND), |res| Ok(Json(res)))
}
pub async fn receive_message(
    State(messages): State<AppState>,
    Json(message): Json<IncomingMessage>,
) {
    messages
        .data
        .lock()
        .unwrap()
        .add(message.content, message.author);
}
