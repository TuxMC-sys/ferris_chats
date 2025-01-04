use super::server::*;
use reqwest::Client;

fn hardcoded_server() -> String {
    String::from("http://localhost:3000/messages/")
}
async fn last_n_messages(number: u32) -> Messages {
    let client = Client::builder().build().unwrap();
    let message_count: u32 = client
        .get(hardcoded_server() + "count")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .parse::<u32>()
        .unwrap();
    client
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
        .await
        .unwrap()
        .json::<Messages>()
        .await
        .unwrap()
}
async fn send(message: String, author: Option<String>) {
    let client = Client::builder().build().unwrap();
    client
        .post(hardcoded_server() + "receive")
        .json(&Message::new(message, author))
        .send()
        .await
        .unwrap();
}
