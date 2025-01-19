mod server;
use crate::server::*;
use axum::{
    routing::{get, post},
    Router,
};
use axum_server::Server;
use core::net::SocketAddr;
use ctrlc::set_handler;
use ferris_chats_data::{AppState, Messages};
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    println!("Starting server. Use ctrl+c to exit and save.");
    let messages = AppState {
        data: Arc::new(Mutex::new(Messages::from_existing_else_new())),
    };
    let app = Router::new()
        .route("/{first}/{amount}", get(get_messages))
        .route("/time/{time}", get(messages_from_time))
        .route("/all", get(all_messages))
        .route("/count", get(message_count))
        .route("/endpoint", post(receive_message))
        .with_state(messages.clone());
    set_handler(move || {
        messages
            .clone()
            .data
            .lock()
            .expect("Mutex poisoned, messages not saved")
            .save_messages();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    Server::bind(SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
