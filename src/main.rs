use axum::{extract::State, routing::get, Router};
use sqlx::{mysql::MySqlPoolOptions, Pool, Row};

#[derive(Clone)]
struct AppState {
    pool: Pool<sqlx::MySql>,
}

fn router() -> Router<AppState> {
    Router::new().route("/", get(handle_get))
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
    let pool = connect().await.expect("connect to database");
    let state = AppState { pool };
    let app = router().with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handle_get(State(state): State<AppState>) {
    println!("GET /");
    // insert some data
    sqlx::query("INSERT INTO mytable (name) VALUES (?)")
        .bind("World")
        .execute(&state.pool)
        .await
        .unwrap();
    // fetch all
    let rows = sqlx::query("SELECT * FROM mytable")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    println!("Got {} rows", rows.len());
    // print all
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("id: {}, name: {}", id, name);
    }
}
