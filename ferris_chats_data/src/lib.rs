use chrono::{DateTime, Utc};
use core::error::Error;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_vec};
use std::ffi::OsString;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
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
    pub fn new(content: String, author: Option<String>) -> Self {
        Self {
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
impl Default for Messages {
    fn default() -> Self {
        Self::new()
    }
}

impl Messages {
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    pub fn from_existing_else_new() -> Self {
        Self::from_messages().unwrap_or_default()
    }
    fn from_messages() -> Result<Self, Box<dyn Error>> {
        Ok(from_str(
            read_to_string(file_in_path(String::from(FILENAME)))?.as_str(),
        )?)
    }
    #[expect(clippy::unwrap_used)]
    pub fn save_messages(&self) {
        _ = create_dir_all(file_in_path(String::new())).is_ok();
        write(file_in_path(String::from(FILENAME)), to_vec(&self).unwrap())
            .expect("Unable to write file");
    }
    pub fn add(&mut self, content: String, author: Option<String>) {
        self.messages.push(Message::new(content, author));
    }
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
    pub fn add_messages(&mut self, new: &mut Self) {
        self.messages.append(&mut new.messages);
    }
    pub fn concat_message(self, message: Message) -> Self {
        let mut messages = self.messages;
        messages.push(message);
        Self { messages }
    }
    pub fn get_range(self, start: usize, end: usize) -> Option<Self> {
        let message_slice = self.messages.get(start..end);
        message_slice.map(|messages| Self {
            messages: messages.to_owned(),
        })
    }
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
    pub fn last_index_at_time(&self, time: DateTime<Utc>) -> Option<usize> {
        self.messages
            .clone()
            .into_iter()
            .enumerate()
            .filter(|x: &(usize, Message)| x.1.time <= time)
            .map(|x: (usize, Message)| x.0)
            .last()
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
