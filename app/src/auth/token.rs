use chrono::{DateTime, Utc};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::db_init;

use crate::AppError;

#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateVerificationToken",
    derive(Serialize, Deserialize),
    omit(id, token)
)]
#[partial(
    "CreateVerificationTokenConent",
    derive(Serialize, Deserialize),
    omit(id)
)]
pub struct VerificationToken {
    pub id: RecordId,
    pub identifier: String,
    pub expires: DateTime<Utc>,
    pub token: String,
}

impl VerificationToken {
    pub async fn create_verification_token(
        token: CreateVerificationToken,
    ) -> Result<VerificationToken, AppError> {
        let client = db_init().await?;

        let content: CreateVerificationTokenConent = CreateVerificationTokenConent {
            identifier: token.identifier,
            expires: token.expires,
            token: Uuid::new_v4().to_string(),
        };

        let result: Option<Self> = client.create("verificationToken").content(content).await?;

        match result {
            Some(token) => Ok(token),
            None => Err(AppError::AuthError(
                "Could not create verification token".into(),
            )),
        }
    }

    pub async fn use_verification_token(
        identifier: String,
        token: String,
    ) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut result = client.query(
                "DELETE verificationToken WHERE identifier = $identifier AND token = $tokenstring RETURN BEFORE;"
            )
            .bind(("identifier", identifier))
            .bind( ("tokenstring", token)).await?;

        let token: Option<Self> = result.take(0)?;

        tracing::info!("use_verification_token result: {:#?}", token);

        match token {
            Some(token) => {
                if token.expires < Utc::now() {
                    return Err(AppError::AuthError("Token has expired".into()));
                }
                Ok(token)
            }
            None => return Err(AppError::AuthError("Token not found".into())),
        }
    }
}
