use axum::{
    routing::{delete, get, post, put},
    Router,
};

use handler::{handle_sign_in, handle_sign_up};
use sqlx::{mysql::MySqlPoolOptions, Pool};

use crate::handler::{
    handle_create_todo, handle_delete_todo, handle_get_all_todos, handle_get_todo_by_id,
    handle_update_todo,
};

mod handler;
mod jwt;

#[derive(Clone)]
struct AppState {
    pool: Pool<sqlx::MySql>,
}

fn todo_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_all_todos))
        .route("/todo", post(handle_create_todo))
        .route("/todo/:id", put(handle_update_todo))
        .route("/todo/:id", delete(handle_delete_todo))
        .route("/todo/:id", get(handle_get_todo_by_id))
}

fn sign_router() -> Router<AppState> {
    Router::new()
        .route("/signUp", post(handle_sign_up))
        .route("/signIn", post(handle_sign_in))
}

async fn connect() -> Result<Pool<sqlx::MySql>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://myuser:mypassword@mariadb/mydatabase")
        .await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect().await.expect("database should connect");
    let state = AppState { pool };
    let app = Router::new()
        .merge(todo_router())
        .merge(sign_router())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3333")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
