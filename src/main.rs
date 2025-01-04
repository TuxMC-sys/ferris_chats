use crate::mods::data::*;
use crate::mods::server::*;
use axum::{
    routing::{get, post},
    Router,
};
use axum_server::Server;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
mod mods;
#[tokio::main]
async fn main() {
    let messages = AppState {
        data: Arc::new(Mutex::new(Messages::from_existing_else_new().to_owned())),
    };
    let app = Router::new()
        .route("/messages/{first}/{amount}", get(get_messages))
        .route("/messages/time/{time}", get(messages_from_time))
        .route("/messages/all", get(all_messages))
        .route("/messages/count", get(message_count))
        .route("/messages/endpoint", post(receive_message))
        .with_state(messages.clone());
    ctrlc::set_handler(move || {
        messages
            .clone()
            .data
            .lock()
            .expect("Mutex poisoned, messages not saved")
            .save_messages();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
