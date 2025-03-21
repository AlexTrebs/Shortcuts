mod error;
mod models;
mod repository;
mod routes;
mod service;
mod templates;
mod utils;

use axum::{routing::get, Router, response::{Html, IntoResponse}, extract::State};
use routes::{create_routes};
use std::{env::var, sync::{LazyLock, RwLock}};
use templates::get_index;
use tera::{Tera, Context};
use tera_hot_reload::{watch, LiveReloadLayer, TeraTemplate};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use service::shortcut::{ShortcutService, ShortcutServiceTrait};

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    RwLock::new(tera::Tera::new("ui/templates/**/*").expect("Failed to create Tera instance"))
});

pub async fn app() -> Result<Router, anyhow::Error> {
    let shortcut_service = ShortcutService::new(/*repo*/);
    let tera = match Tera::new("ui/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    
    Ok(Router::new()
        .route("/", get(get_index))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/api", create_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(LiveReloadLayer::new())
        .with_state(shortcut_service))
}

pub async fn run() {
    tracing_subscriber::registry()
        .with(create_templates())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "Shortcuts=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));

    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr).await.unwrap();

    let app = app().await.unwrap();

    let _debouncer = watch(
        move || {
            let _ = TERA.write().unwrap().full_reload();
            reloader.reload();
        },
        Duration::from_millis(100), // if you have tailwindcss and your machine is slow, you can increase this value
        vec!["./ui"] // this is now listening for changes in the templates folder add any other folders you want to watch this can be your folder that holds your JS files or CSS or whatever you are serving in your app
    );

    axum::serve(listener, app).await.unwrap();
}