use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct UserUpdate {
    age: Option<u8>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();

    let user = User {
        name: "John".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };

    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/", patch(update_handler))
        .with_state(user);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn index_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    user.lock().unwrap().clone().into()
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(payload): Json<UserUpdate>,
) -> Json<User> {
    let mut user = user.lock().unwrap();
    if let Some(age) = payload.age {
        user.age = age;
    }
    if let Some(skills) = payload.skills {
        user.skills = skills;
    }

    user.clone().into()
}
