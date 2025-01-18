use ferris_chats_data::{Message, Messages};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::{task, time};
async fn init() {
    let mut messages = Arc::new(Mutex::new(last_n_messages(100).await.expect("").clone()));
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            update_state(&mut messages).await.expect("REPLACE ME");
        }
    });
}
async fn update_state(
    messages: &mut Arc<Mutex<Messages>>,
) -> Result<(), Box<dyn std::error::Error>> {
    add_new(messages).await?;
    Ok(())
}
pub async fn last_n_messages(number: usize) -> Result<Messages, Box<dyn std::error::Error>> {
    let client = new_client();
    let message_count = get_len().await?;
    Ok(client
        .await
        .get(
            hardcoded_server()
                + (if message_count - number > 0 {
                    message_count - number
                } else {
                    0
                })
                .to_string()
                .as_str()
                + "/"
                + message_count.to_string().as_str(),
        )
        .send()
        .await?
        .json()
        .await?)
}
async fn send(message: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();
    client
        .await
        .post(hardcoded_server() + "/endpoint")
        .json(message)
        .send()
        .await?;
    Ok(())
}
pub async fn add_new(
    messages: &mut Arc<Mutex<Messages>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut messages = messages.lock().await;
    let count = messages.clone().message_count();
    messages.add_messages(
        &mut last_n_messages(get_len().await.expect("REPLACE ME") - count)
            .await?
            .clone(),
    );
    Ok(())
}
pub async fn get_len() -> Result<usize, Box<dyn std::error::Error>> {
    let client = new_client();
    Ok(client
        .await
        .get(hardcoded_server() + "count")
        .send()
        .await?
        .text()
        .await?
        .parse::<usize>()?)
}
pub async fn send_or_return(
    message: Message,
    extant_messages: Option<Messages>,
) -> Result<(), Messages> {
    if send(&message).await.is_err() {
        Err(match extant_messages {
            Some(extant_messages) => extant_messages.clone().concat_message(message),
            None => Messages::new().concat_message(message),
        })
    } else {
        Ok(())
    }
}
fn hardcoded_server() -> String {
    String::from("http://localhost:3000/messages/")
}
async fn new_client() -> Client {
    Client::builder().build().unwrap()
}
