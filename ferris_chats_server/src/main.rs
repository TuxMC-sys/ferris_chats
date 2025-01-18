#![warn(clippy::restriction, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::allow_attributes_without_reason,
    clippy::wildcard_imports,
    clippy::question_mark_used,
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::single_call_fn,
    clippy::map_err_ignore
)]
extern crate alloc;
mod server;
use crate::server::*;
use alloc::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use axum_server::Server;
use core::net::SocketAddr;
use ctrlc::set_handler;
use ferris_chats_data::{AppState, Messages};
use std::io::Result;
use std::process::exit;
use std::sync::Mutex;
#[tokio::main]
async fn main() -> Result<()> {
    let messages = AppState {
        data: Arc::new(Mutex::new(Messages::from_existing_else_new())),
    };
    let app = Router::new()
        .route("/messages/{first}/{amount}", get(get_messages))
        .route("/messages/time/{time}", get(messages_from_time))
        .route("/messages/all", get(all_messages))
        .route("/messages/count", get(message_count))
        .route("/messages/endpoint", post(receive_message))
        .with_state(messages.clone());
    set_handler(move || {
        messages
            .clone()
            .data
            .lock()
            .expect("Mutex poisoned, messages not saved")
            .save_messages();
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(addr).serve(app.into_make_service()).await
}
