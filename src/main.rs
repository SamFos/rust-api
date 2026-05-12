use axum::{
    extract::{State, Request},
    response::{IntoResponse, Html},
    routing::{get, post},
    Json, Router, Form
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

struct AppState {
    step_state: AtomicI32,
}

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Deserialize)]
struct CreateMessage {
    new_state: i32,
}

// POST /change-state
// When the frontend tells us to change state, we update the atomic int 
// and redirect the browser to the new URL.
async fn change_state(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<CreateMessage>,
) -> impl IntoResponse {
    println!("hit echo - serving file for state {}", payload.new_state);
    
    // 1. Update the atomic state
    state.step_state.store(payload.new_state, Ordering::Relaxed);
    
    // 2. Load the value back out to use in the path string
    let current_step = state.step_state.load(Ordering::Relaxed);
    
    // 3. Construct the path
    let path = format!("../nuxt/.output/public/step/{}/index.html", current_step);
    println!("Serving file from: {}", path);

    // 4. Read and return the file
    match tokio::fs::read_to_string(&path).await {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            axum::http::StatusCode::NOT_FOUND,
            format!("File not found at {}: {}", path, err),
        ).into_response(),
    }
}

// GET /step/*s
async fn get_step(req: Request) -> impl IntoResponse {
    let service = ServeFile::new("../nuxt/.output/public/index.html");
    
    // "Oneshot" runs the service with the current request
    match service.oneshot(req).await {
        Ok(res) => res.into_response(),
        Err(err) => {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", err),
            ).into_response()
        }
    }
}

async fn root() -> Json<Message> {
    Json(Message { message: "/dashboard".to_string() })
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let shared_state = Arc::new(AppState {
        step_state: AtomicI32::new(1),
    });

    let app = Router::new()
        .route("/", get(root))
        // We serve the same index.html for ALL /step/ routes.
        // This lets Nuxt's internal router decide what to show without a loop.
        .route("/step/*path", get(get_step))
        .route("/change-state", post(change_state))
        // Serve static assets (CSS/JS) so the page actually renders
        .nest_service("/_nuxt", ServeDir::new("../nuxt/.output/public/_nuxt"))
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 6666));
    println!("Server running on http://localhost:6666");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
