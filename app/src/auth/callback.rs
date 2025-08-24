use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_query_map;
use urlencoding::decode;

#[derive(Clone, Debug)]
enum AuthStatus {
    Loading,
    Success,
    Error(String),
}

#[component]
pub fn AuthCallback() -> impl IntoView {
    let query = use_query_map();
    let (auth_status, set_auth_status) = signal(AuthStatus::Loading);

    let params = move || {
        let token = query.get().get("token");
        let email = query.get().get("email");
        let callbackurl = query.get().get("callbackUrl");

        (token, email, callbackurl)
    };

    Effect::new(move || {
        let (token, email, callback_url) = params();

        #[cfg(feature = "ssr")]
        // Log the parameters for debugging
        tracing::info!(
            "AuthCallback params - Token: {:?}, Email: {:?}, Callback URL: {:?}",
            token,
            email,
            callback_url
        );

        if let (Some(token), Some(email)) = (token, email) {
            let set_auth_status = set_auth_status.clone();
            spawn_local(async move {
                // Handle URL decode errors
                let email = match decode(email.as_str()) {
                    Ok(decoded) => decoded.to_string(),
                    Err(e) => {
                        set_auth_status
                            .set(AuthStatus::Error(format!("Invalid email parameter: {}", e)));
                        return;
                    }
                };

                let result = verify_token_callback_get_session_token(token, email.clone()).await;

                match result {
                    Ok(_) => {
                        #[cfg(feature = "ssr")]
                        tracing::info!("Token verification successful for email: {}", email);
                        set_auth_status.set(AuthStatus::Success);

                        // store_token(session_token.clone());
                        // set_cookie_session_token(Some(session_token.clone()));

                        // Redirect or update UI as needed
                        if let Some(url) = callback_url {
                            let url = match decode(url.as_str()) {
                                Ok(decoded) => decoded.to_string(),
                                Err(_e) => {
                                    #[cfg(feature = "ssr")]
                                    tracing::error!("Failed to decode callback URL: {}", _e);

                                    return;
                                }
                            };
                            // Perform redirect to callback URL
                            window().location().set_href(&url).unwrap();
                        }
                    }
                    Err(e) => {
                        #[cfg(feature = "ssr")]
                        tracing::error!("Token verification failed: {:?}", e);

                        set_auth_status
                            .set(AuthStatus::Error(format!("Authentication failed: {}", e)));
                    }
                }
            });
        } else {
            set_auth_status.set(AuthStatus::Error(
                "Missing required parameters: token or email".to_string(),
            ));
        }
    });

    view! {
        <div class="h-full flex items-center justify-center bg-neutral-50 dark:bg-neutral-900 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                {move || match auth_status.get() {
                    AuthStatus::Loading => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8 text-center">
                                <div class="flex justify-center mb-4">
                                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                                </div>
                                <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                    "Processing authentication..."
                                </p>
                                <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-400">
                                    "Please wait while we verify your credentials"
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    AuthStatus::Success => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8 text-center">
                                <div class="flex justify-center mb-4">
                                    <div class="rounded-full bg-green-100 dark:bg-green-900/20 p-3">
                                        <svg
                                            class="h-8 w-8 text-green-600 dark:text-green-400"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M5 13l4 4L19 7"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                                <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                    "Authentication successful!"
                                </p>
                                <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-400">"Redirecting..."</p>
                            </div>
                        }
                            .into_any()
                    }
                    AuthStatus::Error(error) => {
                        view! {
                            <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-lg p-8">
                                <div class="flex justify-center mb-4">
                                    <div class="rounded-full bg-red-100 dark:bg-red-900/20 p-3">
                                        <svg
                                            class="h-8 w-8 text-red-600 dark:text-red-400"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M6 18L18 6M6 6l12 12"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                                <div class="text-center">
                                    <p class="text-lg font-medium text-neutral-900 dark:text-neutral-100">
                                        "Authentication failed"
                                    </p>
                                    <p class="mt-2 text-sm text-red-600 dark:text-red-400 break-words">{error}</p>
                                    <p class="mt-4 text-sm text-neutral-600 dark:text-neutral-400">
                                        "Please try signing in again or contact support if the issue persists."
                                    </p>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[server]
pub async fn verify_token_callback_get_session_token(
    token: String,
    email: String,
) -> Result<String, ServerFnError> {
    use http::header::HeaderValue;
    use leptos_axum::ResponseOptions;
    let token = crate::auth::token::VerificationToken::use_verification_token(email, token).await?;
    let user = crate::auth::user::AdapterUser::get_user_by_email(crate::email::EmailAddress(
        token.identifier,
    ))
    .await?;
    let session = user.new_session().await?;

    // Create the cookie
    let cookie = session.build_session_cookie();

    // Set the cookie via ResponseOptions
    if let Some(resp) = use_context::<ResponseOptions>() {
        resp.insert_header(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    Ok(session.session_token)
}
