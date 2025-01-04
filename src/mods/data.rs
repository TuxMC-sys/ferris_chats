use std::ffi::OsString;
use std::fs::{create_dir, read_to_string, write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_vec};
use chrono::{DateTime, Utc};
use dirs::home_dir;

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

    pub fn add(&mut self, content: String, author: Option<String>) {
        self.messages.push(Message::new(content, author));
    }
    pub(crate) fn get_range(self, start: usize, end: usize) -> Option<Self> {
        let message_slice = self.messages.get(start..end);
        message_slice.map(|message_slice| Messages {
            messages: message_slice.to_owned().to_vec(),
        })
    }
    pub(crate) fn message_count(&self) -> usize {
        self.messages.len()
    }
    pub(crate) fn last_index_at_time(&self, time: DateTime<Utc>) -> Option<usize> {
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
