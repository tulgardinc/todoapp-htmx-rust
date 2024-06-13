mod todo;

use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::Response,
    response::{Html, IntoResponse},
    routing::{delete, get, patch, post},
    Error, Form, Json, Router,
};
use maud::html;
use serde::Serialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteConnection},
    Pool, Sqlite,
};
use todo::todo_component;
use tokio::time::sleep;
use tower_http::services::ServeDir;

async fn root() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

#[derive(serde::Deserialize)]
struct NewTodo {
    name: String,
}

async fn add_todo(
    State(pool): State<Pool<Sqlite>>,
    Form(new_todo): Form<NewTodo>,
) -> impl IntoResponse {
    println!("reached");

    let mut connection = pool.acquire().await.unwrap();

    let result = sqlx::query!("INSERT INTO todos (name) VALUES (?)", new_todo.name)
        .execute(&mut *connection)
        .await
        .unwrap();

    todo_component(Todo {
        id: result.last_insert_rowid(),
        name: new_todo.name,
        checked: false,
    })
}

#[derive(Debug)]
struct DBTodo {
    id: i64,
    name: String,
    checked: i64,
}

#[derive(Serialize, Clone, Debug)]
pub struct Todo {
    id: i64,
    name: String,
    checked: bool,
}

impl From<DBTodo> for Todo {
    fn from(db_todo: DBTodo) -> Self {
        Todo {
            id: db_todo.id,
            name: db_todo.name,
            checked: db_todo.checked != 0,
        }
    }
}

async fn get_todos(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let db_todos = sqlx::query_as!(DBTodo, "SELECT * FROM todos ORDER BY id")
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    println!("{:?}", db_todos.get(0));

    let todos: Vec<Todo> = db_todos.into_iter().map(Todo::from).collect();

    sleep(Duration::from_millis(1000)).await;

    html! {
        @for todo in todos {
            (todo_component(todo))
        }
    }
}

async fn toggle_todo(State(pool): State<Pool<Sqlite>>, Path(id): Path<i64>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let mut todo: Todo = sqlx::query_as!(DBTodo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_one(&mut *conn)
        .await
        .unwrap()
        .into();

    let val = if todo.checked { 0 } else { 1 };

    sqlx::query!("UPDATE todos SET checked=? WHERE id = ?", val, id)
        .execute(&mut *conn)
        .await
        .unwrap();

    todo.checked = !todo.checked;

    todo_component(todo)
}

async fn delete_todo(State(pool): State<Pool<Sqlite>>, Path(id): Path<i64>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&mut *conn)
        .await
        .unwrap();

    "Deleted Todo item"
}

#[tokio::main]
async fn main() {
    let pool = sqlx::SqlitePool::connect("sqlite://./todos.db")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/todos", post(add_todo))
        .route("/api/todos", get(get_todos))
        .route("/api/todos/:id", patch(toggle_todo))
        .route("/api/todos/:id", delete(delete_todo))
        .nest_service("/public", ServeDir::new("./public"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
