#[cfg(feature = "ssr")]
use crate::auth::{
    account::LinkAccountData,
    session::{AdapterSession, CreateSessionData, UpdateSessionData},
    token::{CreateVerificationToken, VerificationToken},
    user::{AdapterUser, CreateUserData, UpdateUserData},
};

use crate::AppError;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

pub struct SurrealAdapter {}

#[cfg(feature = "ssr")]
impl SurrealAdapter {
    pub async fn create_user(&self, user_data: CreateUserData) -> Result<AdapterUser, AppError> {
        Ok(AdapterUser::create_user(user_data).await?)
    }

    pub async fn get_user(&self, id: RecordId) -> Result<AdapterUser, AppError> {
        Ok(AdapterUser::get_user(id).await?)
    }

    pub async fn get_user_by_account(
        &self,
        provider_account_id: RecordId,
    ) -> Result<Option<AdapterUser>, AppError> {
        Ok(AdapterUser::get_user_by_account(provider_account_id).await?)
    }

    pub async fn update_user(data: UpdateUserData) -> Result<AdapterUser, AppError> {
        let updated = AdapterUser::update_user(data).await?;
        Ok(updated)
    }

    pub async fn delete_user(&self, id: RecordId) -> Result<(), AppError> {
        let user = AdapterUser::get_user(id).await?;
        user.delete_user().await?;
        Ok(())
    }

    pub async fn link_account(&self, account: LinkAccountData) -> Result<(), AppError> {
        let client = crate::db_init().await?;

        // let account_data = serde_json::json!({
        //     "type": account.account_type,
        //     "access_token": account.access_token,
        //     "expires_at": account.expires_at,
        //     "userId": account.user_id,
        //     "providerAccountId": account.provider_account_id,
        //     "scope": account.scope,
        //     "provider": account.provider,
        //     "token_type": account.token_type,
        //     "refresh_token": account.refresh_token,
        // });

        // let query = format!(
        //     "CREATE ONLY account CONTENT {} RETURN AFTER;",
        //     serde_json::to_string(&account_data)?
        // );

        let _: Option<LinkAccountData> = client.create("account").content(account).await?;

        Ok(())
    }

    pub async fn unlink_account(&self, provider_account_id: &str) -> Result<(), AppError> {
        let client = crate::db_init().await?;
        let query = format!(
            "DELETE account WHERE providerAccountId = '{}';",
            provider_account_id
        );
        client.query(&query).await?;
        Ok(())
    }

    pub async fn create_session(
        session_data: CreateSessionData,
    ) -> Result<AdapterSession, AppError> {
        let session = AdapterSession::create_session(session_data).await?;
        Ok(session)
    }

    pub async fn get_session_and_user(
        &self,
        session_token: String,
    ) -> Result<Option<(AdapterSession, AdapterUser)>, AppError> {
        let session = AdapterSession::from_string(session_token).await?;
        let user = AdapterUser::get_user(session.user_id.clone()).await?;
        Ok(Some((session, user)))
    }

    pub async fn update_session(
        // &self,
        data: UpdateSessionData,
    ) -> Result<Option<AdapterSession>, AppError> {
        let updated = AdapterSession::update_session(data).await?;
        Ok(updated)
    }

    pub async fn delete_session(session_token: String) -> Result<Option<AdapterSession>, AppError> {
        let result = AdapterSession::delete_session(session_token).await?;
        Ok(result)
    }

    pub async fn create_verification_token(
        &self,
        token: CreateVerificationToken,
    ) -> Result<VerificationToken, AppError> {
        let token = VerificationToken::create_verification_token(token).await?;
        Ok(token)
    }

    pub async fn use_verification_token(
        &self,
        identifier: String,
        token: String,
    ) -> Result<VerificationToken, AppError> {
        let token = VerificationToken::use_verification_token(identifier, token).await?;
        Ok(token)
    }
}

#[cfg(feature = "ssr")]
#[tokio::test]
#[ignore = "temporary skip for cicd test"]
async fn test_adapter() -> Result<(), AppError> {
    use crate::theme::Theme;
    use chrono::Utc;

    // use crate::components::theme::Theme;

    let db = crate::db_init().await?;

    let testname = "test_user";

    db.query("DELETE user WHERE name ~ $name;")
        .bind(("name", testname))
        .await?;

    println!("Available in tests or when SSR feature is enabled");

    let adapter = SurrealAdapter {};

    let user_to_create = CreateUserData {
        name: testname.to_string(),
        image: None,
        theme: Theme::default(),
    };

    let newuser = adapter.create_user(user_to_create.clone()).await?;
    println!("Created User: {:?}", newuser);

    let should_error = adapter.create_user(user_to_create).await;
    assert!(
        should_error.is_err(),
        "Should not create user with same email"
    );

    // // test verification token
    // println!("creating new verification token");
    // let new_token = newuser.new_verification_token().await?;

    // println!("checking new token");

    // let check_token = VerificationToken::use_verification_token(
    //     newuser.email.to_string(),
    //     new_token.token.clone(),
    // )
    // .await?;

    // // set verified email
    // let updated_user = newuser.set_verified_email().await?;

    // assert!(updated_user.email_verified.is_some());

    // println!("{:?}", check_token);
    // assert_eq!(check_token.identifier, newuser.email.to_string());
    // assert!(check_token.expires > Utc::now());
    // assert_eq!(check_token.token.clone(), new_token.token.clone());

    // // test session

    // let newsession = newuser.new_session().await?;
    // println!("Created Session: {:?}", newsession);

    // let session_from_string = AdapterSession::from_string(newsession.session_token.clone()).await?;

    // assert_eq!(session_from_string.session_token, newsession.session_token);

    // let user_from_session: AdapterUser =
    //     AdapterUser::get_user(session_from_string.user_id.clone()).await?;

    // assert_eq!(user_from_session.id, newuser.id);
    // assert_eq!(user_from_session.id, updated_user.id);

    // // optimization - get user from session directly

    // let user_fast = AdapterUser::get_user_from_session(newsession.session_token.clone()).await?;

    // assert_eq!(user_fast.id, newuser.id);

    Ok(())
}
