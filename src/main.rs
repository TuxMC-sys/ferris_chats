use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_server::Server;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<Messages>>,
}
#[derive(Default, Deserialize, Serialize, Clone)]
struct Message {
    content: String,
    author: Option<String>,
    time: Option<String>,
}
impl Message {
    fn new(content: String, author: Option<String>) -> Message {
        Message {
            content,
            author: Some(author.unwrap_or_else(|| String::from("Unknown"))),
            time: Some(Utc::now().to_rfc3339()),
        }
    }
}
#[derive(Serialize, Clone)]
struct Messages {
    messages: Vec<Message>,
}
impl Messages {
    fn new() -> Self {
        Messages {
            messages: Vec::new(),
        }
    }
    fn add(&mut self, message: Message) {
        self.messages.push(message);
    }
    fn get_range(self, start: usize, end: usize) -> Option<Self> {
        let message_slice = self.messages.get(start..end);
        match message_slice {
            Some(message_slice) => Some(Messages {
                messages: message_slice.to_owned().to_vec(),
            }),
            None => None,
        }
    }
    fn message_count(&self) -> usize {
        self.messages.len()
    }
}

#[tokio::main]
async fn main() {
    let messages = AppState {
        data: Arc::new(Mutex::new(Messages::new().to_owned())),
    };
    let app = Router::new()
        .route(
            "/messages/{first}/{amount}",
            get(get_messages).with_state(messages.clone()),
        )
        .route(
            "/messages/all",
            get(all_messages).with_state(messages.clone()),
        )
        .route(
            "/messages/count",
            get(message_count).with_state(messages.clone()),
        )
        .route("/message/receive", post(receive_message).with_state(messages));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn get_messages(
    State(messages): State<AppState>,
    Path((first, end)): Path<(usize, usize)>,
) -> Result<Json<Messages>, StatusCode> {
    match <Messages as Clone>::clone(&messages.data.lock().unwrap()).get_range(first, end) {
        Some(res) => Ok(Json(res)),
        None => Err(StatusCode::BAD_REQUEST),
    }
}
async fn all_messages(State(messages): State<AppState>) -> Json<Messages> {
    Json(messages.data.lock().unwrap().clone())
}
async fn message_count(State(messages): State<AppState>) -> String {
    messages.data.lock().unwrap().message_count().to_string()
}
async fn receive_message(State(messages): State<AppState>, Json(message): Json<Message>) {
    messages
        .data
        .lock()
        .unwrap()
        .add(Message::new(message.content, message.author));
}
