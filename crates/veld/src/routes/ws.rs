//! WebSocket endpoint /ws. Ticket 018.

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tracing::debug;

use crate::state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    let rx = state.broadcast_tx.subscribe();
    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(
    socket: WebSocket,
    mut rx: tokio::sync::broadcast::Receiver<crate::broadcast::WsEnvelope>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Forward broadcast messages to the client.
    let send_task = tokio::spawn(async move {
        while let Ok(envelope) = rx.recv().await {
            if let Ok(json) = envelope.to_json() {
                if sender
                    .send(axum::extract::ws::Message::Text(json))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    });

    // Drain incoming messages (we don't process them; just keep connection alive).
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(_msg)) = receiver.next().await {
            debug!("ws message received");
        }
    });

    let _ = tokio::join!(send_task, recv_task);
}
