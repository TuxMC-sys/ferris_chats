use super::data::*;
use reqwest::Client;

fn hardcoded_server() -> String {
    String::from("http://localhost:3000/messages/")
}
async fn last_n_messages(number: u32) -> Result<Messages, Box<dyn std::error::Error>> {
    let client = Client::builder().build().unwrap();
    let message_count: u32 = client
        .get(hardcoded_server() + "count")
        .send()
        .await?
        .text()
        .await?
        .parse::<u32>()?;
    Ok(client
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
async fn send(message: String, author: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build().unwrap();
    client
        .post(hardcoded_server() + "/endpoint")
        .json(&Message::new(message, author))
        .send()
        .await?;
    Ok(())
}
