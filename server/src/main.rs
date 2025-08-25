use crate::tracing_setup::*;
use app::App;
use axum::{extract::OriginalUri, http::Request, Router};
use backend::fallback::file_and_error_handler;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;
use leptos_meta::MetaTags;

use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info_span;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug, axum_macros::FromRef)]
pub struct ServerState {
    pub options: LeptosOptions,
    pub routes: Vec<leptos_axum::AxumRouteListing>,
}
pub mod tracing_setup;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <script>
                    // Prevent white flash by applying theme immediately
                    {r#"
                    (function() {
                        const theme = localStorage.getItem('theme') || 'system';
                        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                        
                        if (theme === 'dark' || (theme === 'system' && prefersDark)) {
                            document.documentElement.classList.add('dark');
                        }
                    })();
                    "#}
                </script>
                <style>
                    {r#"
                    /* Prevent white flash during load */
                    html {
                        background-color: rgb(245 245 245);
                    }
                    html.dark {
                        background-color: rgb(23 23 23);
                    }
                    body {
                        background-color: transparent;
                    }
                    "#}
                </style>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(Some("./Cargo.toml")).unwrap();

    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let meter_provider = init_meter_provider();

    let routes = leptos_axum::generate_route_list(App);

    tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
			"app=debug,frontend=debug,backend=debug,server=debug,tower_http=debug,axum::rejection=trace".into()
		}))
		.with(MetricsLayer::new(meter_provider.clone()))
		.with(OpenTelemetryLayer::new(init_tracer()))
		.with(tracing_subscriber::fmt::layer())
		.init();
    //OtelGuard { meter_provider };

    let _state = ServerState {
        options: leptos_options.clone(),
        routes: routes.clone(),
    };

    let cors = CorsLayer::new()
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_origin(
            "tauri://localhost"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_origin(
            "http://127.0.0.1:80"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_origin(Any)
        .allow_headers(vec![axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let path = if let Some(path) = request.extensions().get::<OriginalUri>() {
                    path.0.path().to_owned()
                } else {
                    request.uri().path().to_owned()
                };
                let remote_addr = request
                    .extensions()
                    .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                    .map(|ci| ci.0);

                info_span!(
                "http_request",
                method = ?request.method(),
                path,
                remote_addr = ?remote_addr,
                )
            }),
        )
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
