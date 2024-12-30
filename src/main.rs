
pub use crate::lib::ferris_server::*;
use axum_server::Server;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use axum::{
    routing::{get, post}, Router,
};
pub mod lib;
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

