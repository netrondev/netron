use crate::email::EmailAddress;

use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::auth::{
    session::{AdapterSession, CreateSessionData},
    token::{CreateVerificationToken, VerificationToken},
    // wallet::Wallet,
};

#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(feature = "ssr")]
use chrono::Utc;

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

use crate::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateUserData",
    derive(Serialize, Deserialize, Clone),
    omit(id, superadmin)
)]
#[partial(
    "UpdateUserData",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(superadmin)
)]
pub struct AdapterUser {
    pub id: RecordId,
    pub name: String,
    pub email_verified: Option<Datetime>,
    pub image: Option<String>,
    pub email: EmailAddress,
    pub superadmin: Option<bool>,
    #[serde(default)]
    pub theme: Theme,
}

#[cfg(feature = "ssr")]
use crate::db::db_init;

#[cfg(feature = "ssr")]
impl AdapterUser {
    pub async fn create_user(user_data: CreateUserData) -> Result<Self, AppError> {
        use crate::db_init;

        let client = db_init().await?;
        let create_result: Option<Self> = client.create("user").content(user_data).await?;
        let created: Self =
            create_result.ok_or_else(|| AppError::AuthError("Could not create user".into()))?;
        Ok(created)
    }

    pub async fn create_test_user() -> Result<Self, AppError> {
        let user = Self::create_user(CreateUserData {
            email: EmailAddress::create_test_email(),
            email_verified: None,
            name: "Test User".to_string(),
            image: None,
            theme: Theme::System,
        })
        .await?;
        Ok(user)
    }

    pub fn is_super_admin(&self) -> Result<bool, AppError> {
        if let Some(superadmin) = self.superadmin {
            Ok(superadmin)
        } else {
            Err(AppError::AuthError("User is not a super admin".into()))
        }
    }

    pub async fn get_user(id: RecordId) -> Result<Self, AppError> {
        let client = db_init().await?;

        if id.table() != "user" {
            return Err(AppError::AuthError("Invalid user ID".into()));
        }

        let result: Option<Self> = client.select(id).await?;

        match result {
            Some(user) => Ok(user),
            None => Err(AppError::AuthError("User not found".into())),
        }
    }

    pub async fn get_user_by_email(email: EmailAddress) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query("SELECT * FROM ONLY user WHERE email = $email LIMIT 1;")
            .bind(("email", email))
            .await?;

        let user: Option<Self> = result.take(0)?;

        match user {
            Some(user) => Ok(user),
            None => Err(AppError::AuthError("User not found".into())),
        }
    }

    pub async fn get_by_email(email: String) -> Result<Self, AppError> {
        use std::str::FromStr;
        let email_address = EmailAddress::from_str(&email)
            .map_err(|_| AppError::AuthError("Invalid email address".into()))?;
        Self::get_user_by_email(email_address).await
    }

    pub async fn get_user_by_account(
        provider_account_id: RecordId,
    ) -> Result<Option<AdapterUser>, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query(
                "SELECT * FROM ONLY account WHERE providerAccountId = $providerAccountId LIMIT 1;",
            )
            .bind(("providerAccountId", provider_account_id))
            .await?;

        let user: Option<Self> = result.take(0)?;

        Ok(user)
    }

    // convert = r##"{ format!("{:#?}", query) }"##,
    // #[cached::proc_macro::cached(
    //     name = "get_user_from_session",
    //     result = true,
    //     convert = r#"{ session_token }"#,
    //     key = "String",
    //     time = 60
    // )]
    pub async fn get_user_from_session(session_token: String) -> Result<Self, AppError> {
        use crate::db::db_seperate_connection;

        let client = db_seperate_connection().await?;

        let mut result = client
            .query("(SELECT user_id from ONLY session where session_token = $session_token LIMIT 1 FETCH user_id).user_id;")
            .bind(("session_token", session_token))
            .await?;

        let user: Option<Self> = result.take(0)?;

        if let Some(user) = user {
            Ok(user)
        } else {
            Err(AppError::AuthError(
                "User not found for session_token".into(),
            ))
        }
    }

    pub async fn set_verified_email(&self) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut user_update = client
            .query("UPDATE $userid SET email_verified = $email_verified RETURN AFTER;")
            .bind(("userid", self.id.clone()))
            .bind(("email_verified", Datetime::from(Utc::now())))
            .await?;

        let user: Option<Self> = user_update.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_user(data: UpdateUserData) -> Result<AdapterUser, AppError> {
        let db = db_init().await?;

        let mut query = db
            .query("UPDATE $userid SET name = $name, email = $email, image = $image RETURN AFTER;")
            .bind(("userid", data.id.clone()))
            .bind(("name", data.name))
            .bind(("email", data.email.to_string()))
            .bind(("image", data.image))
            .await?;

        let user: Option<Self> = query.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_user_theme(&self, theme: Theme) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut query = client
            .query("UPDATE $userid SET theme = $theme RETURN AFTER;")
            .bind(("userid", self.id.clone()))
            .bind(("theme", theme))
            .await?;

        let user: Option<Self> = query.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_user_image(&self, image: String) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut query = client
            .query("UPDATE $userid SET image = $image RETURN AFTER;")
            .bind(("userid", self.id.clone()))
            .bind(("image", image))
            .await?;

        let user: Option<Self> = query.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn delete_user(&self) -> Result<(), AppError> {
        let client = db_init().await?;
        let _: Option<AdapterUser> = client.delete(&self.id).await?;
        // delete all related data?
        Ok(())
    }

    /// Creates a new verification token for the user.
    pub async fn new_verification_token(&self) -> Result<VerificationToken, AppError> {
        let token = VerificationToken::create_verification_token(CreateVerificationToken {
            identifier: self.email.to_string(),
            expires: Utc::now() + chrono::Duration::hours(1),
        })
        .await?;

        Ok(token)
    }

    pub async fn new_session(&self) -> Result<AdapterSession, AppError> {
        let session_data = CreateSessionData {
            user_id: self.id.clone(),
            session_token: uuid::Uuid::new_v4().to_string(),
            expires: Datetime::from(Utc::now() + chrono::Duration::days(365)),
        };

        AdapterSession::create_session(session_data).await
    }

    pub async fn get_all_users() -> Result<Vec<Self>, AppError> {
        let client = db_init().await?;
        let users: Vec<Self> = client.select("user").await?;
        Ok(users)
    }

    // pub async fn wallets(&self) -> Result<Vec<Wallet>, AppError> {
    //     Wallet::get_by_user(self.id.clone()).await
    // }
}
