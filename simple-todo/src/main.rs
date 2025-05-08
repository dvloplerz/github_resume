// main.rs
#![allow(unused)]

use core::panic;
use std::{
    default,
    fmt::Display,
    iter,
    ops::{Deref, DerefMut},
    sync::{Arc, TryLockError},
};

use axum::{
    Json, Router, error_handling,
    extract::{State, rejection::JsonDataError},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    serve,
};

use tokio::sync::Mutex;

pub mod models;
use models::todo_model::Todo;
use serde::{Deserialize, Serialize, de::Error};
use uuid::{Uuid, uuid};

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        todos: Arc::new(Mutex::new(vec![])),
    };
    // Init Axum Runtime.
    let todo_app = Router::new()
        .route("/", get(|| async { (Redirect::permanent("/todo_list")) }))
        .route("/todo_list", get(get_todo))
        .route("/new", post(new_todo));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, todo_app.with_state(state))
        .await
        .unwrap();
}

async fn get_todo(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().await;
    Json(todos.to_vec())
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
    completed: bool,
}
async fn new_todo(
    State(state): State<AppState>, Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let new_todo = Todo::new(&payload.title, payload.completed);
    state.todos.lock().await.push(new_todo.clone());

    Json(new_todo)
}
