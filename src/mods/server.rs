use crate::mods::data::{AppState, IncomingMessage, Messages};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};

pub async fn get_messages(
    State(messages): State<AppState>,
    Path((first, end)): Path<(usize, usize)>,
) -> Result<Json<Messages>, StatusCode> {
    match <Messages as Clone>::clone(&messages.data.lock().unwrap()).get_range(first, end) {
        Some(res) => Ok(Json(res)),
        None => Err(StatusCode::BAD_REQUEST),
    }
}
pub async fn all_messages(State(messages): State<AppState>) -> Json<Messages> {
    Json(messages.data.lock().unwrap().clone())
}
pub async fn message_count(State(messages): State<AppState>) -> String {
    messages.data.lock().unwrap().message_count().to_string()
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
    match messages
        .clone()
        .get_range(index, messages.message_count() + 1)
    {
        Some(res) => Ok(Json(res)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
