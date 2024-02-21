use axum::extract::{Path, State};
use serde::Deserialize;
use sqlx::Row;

use crate::AppState;

pub async fn handle_get_all_todos(State(state): State<AppState>) {
    println!("GET /");
    // fetch all
    let rows = sqlx::query("SELECT * FROM todotable")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    println!("Got {} rows", rows.len());
    // print all
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let is_done: bool = row.get("is_done");
        println!("id: {}, name: {}, is_done: {}", id, name, is_done);
    }
}

#[derive(Debug, Deserialize)]
pub struct Param {
    id: i32,
}

pub async fn handle_get_todo_by_id(State(state): State<AppState>, Path(param): Path<Param>) {
    println!("GET /todo/:id");
    let todo_id = param.id;
    let row = sqlx::query("SELECT * FROM todotable WHERE id = ?")
        .bind(todo_id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let id: i32 = row.get("id");
    let name: String = row.get("name");
    let is_done: bool = row.get("is_done");
    println!("id: {}, name: {}, is_done: {}", id, name, is_done);
}

pub async fn handle_create_todo(State(state): State<AppState>) {
    println!("POST /create");
    // insert some data
    sqlx::query("INSERT INTO todotable (name) VALUES (?)")
        .bind("go to the gym")
        .execute(&state.pool)
        .await
        .unwrap();
}

pub async fn handle_update_todo(State(state): State<AppState>, Path(param): Path<Param>) {
    println!("PUT /todo/:id");
    let todo_id = param.id;
    sqlx::query("UPDATE todotable SET is_done = ? WHERE id = ?")
        .bind(true)
        .bind(todo_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

pub async fn handle_delete_todo(State(state): State<AppState>, Path(param): Path<Param>) {
    println!("DELETE /todo/:id");
    let todo_id = param.id;
    sqlx::query("DELETE FROM todotable WHERE id = ?")
        .bind(todo_id)
        .execute(&state.pool)
        .await
        .unwrap();
}
