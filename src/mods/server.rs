use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_vec};
use std::fs::{create_dir, read_to_string};
use std::sync::{Arc, Mutex};
use std::{ffi::OsString, fs::write, path::PathBuf};

static FILENAME: &str = "messages.json";
#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<Messages>>,
}
#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Message {
    content: String,
    author: Option<String>,
    time: DateTime<Utc>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct IncomingMessage {
    pub content: String,
    pub author: Option<String>,
}
impl Message {
    pub fn new(content: String, author: Option<String>) -> Message {
        Message {
            content,
            author: Some(author.unwrap_or_else(|| String::from("Unknown"))),
            time: Utc::now(),
        }
    }
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Messages {
    pub messages: Vec<Message>,
}
impl Messages {
    pub fn new() -> Self {
        Messages {
            messages: Vec::new(),
        }
    }
    pub fn from_existing_else_new() -> Self {
        Self::from_messages().unwrap_or(Messages::new())
    }
    fn from_messages() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(from_str(
            read_to_string(file_in_path(String::from(FILENAME)))?.as_str(),
        )?)
    }
    pub fn save_messages(&self) {
        let _ = create_dir(file_in_path(String::from(""))).is_ok();
        write(file_in_path(String::from(FILENAME)), to_vec(&self).unwrap())
            .expect("Unable to write file");
    }

    fn add(&mut self, content: String, author: Option<String>) {
        self.messages.push(Message::new(content, author));
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
pub fn file_in_path(file_name: String) -> OsString {
    PathBuf::from(
        &[
            home_dir()
                .unwrap_or_else(|| "".into())
                .display()
                .to_string(),
            String::from("/.ferris_chats/"),
            file_name,
        ]
        .join(""),
    )
    .into_os_string()
}
