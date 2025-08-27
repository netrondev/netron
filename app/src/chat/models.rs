use crate::auth::{session::get_user, user::AdapterUser};

#[cfg(feature = "ssr")]
use crate::{db_init, AppError};

#[cfg(feature = "ssr")]
use chrono::Utc;

use crate::{Datetime, RecordId};
use leptos::prelude::*;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEventType {
    Message,
    UserJoined,
    UserLeft,
}

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateChatEvent",
    derive(Serialize, Deserialize, Clone),
    omit(id, username)
)]
pub struct ChatEventDb {
    pub id: RecordId,
    pub user_id: Option<RecordId>,
    pub username: String,
    pub event_type: ChatEventType,
    pub message: Option<String>,
    pub timestamp: Datetime,
}

#[server]
pub async fn get_chat_history(limit: Option<usize>) -> Result<Vec<ChatEventDb>, ServerFnError> {
    // Verify user is authenticated
    let _user = get_user().await?;

    let db = db_init().await?;
    let limit = limit.unwrap_or(100);

    let query = r#"SELECT * FROM chat_event WHERE event_type in ["Message"] ORDER BY timestamp DESC LIMIT $limit;"#;
    let mut result = db.query(query).bind(("limit", limit)).await?;

    let events: Vec<ChatEventDb> = result.take(0)?;

    // Return in chronological order
    let mut events = events;
    events.reverse();

    Ok(events)
}

#[cfg(feature = "ssr")]
pub async fn save_chat_event(
    user_id: Option<RecordId>,
    username: String,
    event_type: ChatEventType,
    message: Option<String>,
) -> Result<ChatEventDb, AppError> {
    let db = db_init().await?;

    let event_data = CreateChatEvent {
        user_id,
        event_type,
        message,
        timestamp: Datetime::from(Utc::now()),
    };

    let created: Option<ChatEventDb> = db.create("chat_event").content(event_data).await?;
    let created =
        created.ok_or_else(|| AppError::new("Failed to create chat event".to_string()))?;

    Ok(created)
}

#[server]
pub async fn get_user_info(user_id: RecordId) -> Result<Option<AdapterUser>, ServerFnError> {
    // Verify user is authenticated
    let _current_user = get_user().await?;

    match AdapterUser::get_user(user_id).await {
        Ok(user) => Ok(Some(user)),
        Err(_) => Ok(None),
    }
}
