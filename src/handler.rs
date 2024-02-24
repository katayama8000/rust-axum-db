use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::Row;

use crate::AppState;

pub async fn handle_get_all_todos(State(state): State<AppState>) -> impl IntoResponse {
    println!("GET /");

    // fetch all todos
    let rows = match sqlx::query("SELECT * FROM todotable")
        .fetch_all(&state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to fetch todos: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todos").into_response();
        }
    };

    println!("Got {} todos", rows.len());

    // print all todos
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let is_done: bool = row.get("is_done");
        let user_id: i32 = row.get("user_id");
        println!(
            "id: {}, name: {}, is_done: {}, user_id: {}",
            id, name, is_done, user_id
        );
    }

    (StatusCode::OK).into_response()
}

#[derive(Debug, Deserialize)]
pub struct Param {
    id: i32,
}

pub async fn handle_get_todo_by_id(
    State(state): State<AppState>,
    Path(param): Path<Param>,
) -> impl IntoResponse {
    println!("GET /todo/:id");
    let todo_id = param.id;

    // fetch todo by id
    let row = match sqlx::query("SELECT * FROM todotable WHERE id = ?")
        .bind(todo_id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Failed to fetch todo: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todo").into_response();
        }
    };

    let id: i32 = row.get("id");
    let name: String = row.get("name");
    let is_done: bool = row.get("is_done");
    println!("id: {}, name: {}, is_done: {}", id, name, is_done);

    (StatusCode::OK).into_response()
}

pub async fn handle_create_todo(State(state): State<AppState>) -> impl IntoResponse {
    println!("POST /create");

    // For demonstration, let's assume the user ID is known or provided somehow.
    let user_id = 1; // This should be replaced with the actual user ID from the request or session

    // Insert a todo with the user ID
    match sqlx::query("INSERT INTO todotable (name, user_id) VALUES (?, ?)")
        .bind("go to the gym")
        .bind(user_id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            eprintln!("Failed to create todo: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn handle_update_todo(
    State(state): State<AppState>,
    Path(param): Path<Param>,
) -> impl IntoResponse {
    println!("PUT /todo/:id");
    let todo_id = param.id;

    // update todo status
    match sqlx::query("UPDATE todotable SET is_done = ? WHERE id = ?")
        .bind(true)
        .bind(todo_id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("Failed to update todo: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn handle_delete_todo(
    State(state): State<AppState>,
    Path(param): Path<Param>,
) -> impl IntoResponse {
    println!("DELETE /todo/:id");
    let todo_id = param.id;

    // delete todo
    match sqlx::query("DELETE FROM todotable WHERE id = ?")
        .bind(todo_id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("Failed to delete todo: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

pub async fn handle_sign_up(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    println!("POST /signUp");
    println!("name: {}, password: {}", user.name, user.password);

    // insert user
    match sqlx::query("INSERT INTO usertable (name, password) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.password)
        .execute(&state.pool)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn handle_sign_in(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    println!("POST /signIn");
    println!("name: {}, password: {}", user.name, user.password);

    // fetch all users from database
    let rows = match sqlx::query("SELECT * FROM usertable")
        .fetch_all(&state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to fetch users: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users").into_response();
        }
    };
    println!("Got {} users", rows.len());

    // search for the user
    let mut user_found = false;
    for row in rows {
        let name: String = row.get("name");
        let password: String = row.get("password");
        if name == user.name && password == user.password {
            user_found = true;
            break;
        }
    }

    if user_found {
        (StatusCode::OK).into_response()
    } else {
        (StatusCode::UNAUTHORIZED).into_response()
    }
}
