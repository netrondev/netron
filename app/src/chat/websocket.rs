use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::auth::user::AdapterUser;

use super::models::{save_chat_event, ChatEventType};
use super::shared::{ChatMessage, WsMessage};

type Clients = Arc<DashMap<String, String>>;
type Broadcaster = Arc<broadcast::Sender<String>>;

#[derive(Clone)]
pub struct ChatState {
    clients: Clients,
    broadcaster: Broadcaster,
}

impl ChatState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            clients: Arc::new(DashMap::new()),
            broadcaster: Arc::new(tx),
        }
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<ChatState>,
    jar: CookieJar,
) -> impl IntoResponse {
    // Try to get user from session
    let user_result = async {
        let session_cookie = jar
            .iter()
            .find(|cookie| cookie.name().contains("session_token"))
            .ok_or("No session token found")?;

        AdapterUser::get_user_from_session(session_cookie.value().to_string())
            .await
            .map_err(|e| format!("Failed to get user: {}", e))
    }
    .await;

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_result.ok()))
}

async fn handle_socket(socket: WebSocket, state: ChatState, user: Option<AdapterUser>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.broadcaster.subscribe();

    let (client_id, username) = match &user {
        Some(user_data) => {
            // Use actual user ID and name
            let client_id = user_data.id.to_string();
            let username = user_data.name.clone();
            (client_id, username)
        }
        None => {
            // Fallback for unauthenticated users
            warn!("Unauthenticated user connected to chat");
            let client_id = format!("anon_{}", chrono::Utc::now().timestamp_millis());
            let username = format!("Anonymous{}", &client_id[5..9]);
            (client_id, username)
        }
    };

    state.clients.insert(client_id.clone(), username.clone());

    // Save join event to database
    let user_id_for_save = user.as_ref().map(|u| u.id.clone());
    if let Err(e) = save_chat_event(
        user_id_for_save,
        username.clone(),
        ChatEventType::UserJoined,
        None,
    )
    .await
    {
        warn!("Failed to save join event: {}", e);
    }

    let join_msg = WsMessage::UserJoined {
        username: username.clone(),
    };
    let _ = state
        .broadcaster
        .send(serde_json::to_string(&join_msg).unwrap());

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let state_clone = state.clone();
    let username_clone = username.clone();
    let user_id_clone = user.as_ref().map(|u| u.id.clone());
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Save message to database
                if let Err(e) = save_chat_event(
                    user_id_clone.clone(),
                    username_clone.clone(),
                    ChatEventType::Message,
                    Some(text.to_string()),
                )
                .await
                {
                    warn!("Failed to save chat message: {}", e);
                }

                let chat_msg = ChatMessage {
                    user_id: user_id_clone.clone().unwrap_or_else(|| {
                        use surrealdb::RecordId;
                        RecordId::from(("user", "anonymous"))
                    }),
                    username: username_clone.clone(),
                    message: text.to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                let ws_msg = WsMessage::Message(chat_msg);
                if let Ok(json) = serde_json::to_string(&ws_msg) {
                    let _ = state_clone.broadcaster.send(json);
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    info!("Client {} disconnected", &client_id);
    state.clients.remove(&client_id);

    // Save leave event to database
    let user_id_for_leave = user.as_ref().map(|u| u.id.clone());
    if let Err(e) = save_chat_event(
        user_id_for_leave,
        username.clone(),
        ChatEventType::UserLeft,
        None,
    )
    .await
    {
        warn!("Failed to save leave event: {}", e);
    }

    let leave_msg = WsMessage::UserLeft { username };
    let _ = state
        .broadcaster
        .send(serde_json::to_string(&leave_msg).unwrap());
}

pub fn chat_routes() -> axum::Router<ChatState> {
    axum::Router::new().route("/ws", axum::routing::get(websocket_handler))
}
