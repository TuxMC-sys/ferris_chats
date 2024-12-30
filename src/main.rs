use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_server::Server;
use chrono::{DateTime, Utc};
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
    time: DateTime<Utc>,
}
impl Message {
    fn new(content: String, author: Option<String>) -> Message {
        Message {
            content,
            author: Some(author.unwrap_or_else(|| String::from("Unknown"))),
            time: Utc::now(),
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
        message_slice.map(|message_slice| Messages {
            messages: message_slice.to_owned().to_vec(),
        })
    }
    fn message_count(&self) -> usize {
        self.messages.len()
    }
    fn last_index_at_time(&self, time: DateTime<Utc>) -> Option<usize> {
        self.messages
            .clone()
            .into_iter()
            .enumerate()
            .filter(|x: &(usize, Message)| x.1.time <= time)
            .map(|x: (usize, Message)| x.0)
            .last()
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
            "/messages/time/{time}",
            get(messages_from_time).with_state(messages.clone()),
        )
        .route(
            "/messages/all",
            get(all_messages).with_state(messages.clone()),
        )
        .route(
            "/messages/count",
            get(message_count).with_state(messages.clone()),
        )
        .route(
            "/message/receive",
            post(receive_message).with_state(messages),
        );
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
async fn messages_from_time(
    State(messages): State<AppState>,
    Path(time): Path<String>,
) -> Result<Json<Messages>, StatusCode> {
    let messages = messages.data.lock().unwrap().clone();
    let time = match time.parse::<DateTime<Utc>>() {
        Ok(t) => t,
        Err(_e) => return Err(StatusCode::BAD_REQUEST),
    };
    let index = match messages.last_index_at_time(time) {
        Some(index) => index,
        None => return Err(StatusCode::NOT_FOUND),
    };
    match messages.clone().get_range(index, messages.message_count()) {
        Some(res) => Ok(Json(res)),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
