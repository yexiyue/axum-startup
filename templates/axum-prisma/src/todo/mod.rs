use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;

use crate::{prisma::todo, Prisma};

pub fn todo_router() -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", put(update_todo).delete(delete_todo))
}

async fn get_todos(prisma: Prisma) -> impl IntoResponse {
    let res = prisma.todo().find_many(vec![]).exec().await.unwrap();
    Json(res)
}

#[derive(Debug, Deserialize)]
struct TodoQuery {
    title: String,
    completed: Option<bool>,
}

async fn create_todo(prisma: Prisma, Json(todo): Json<TodoQuery>) -> impl IntoResponse {
    let res = prisma
        .todo()
        .create(todo.title.clone(), vec![])
        .exec()
        .await
        .unwrap();
    Json(res)
}

async fn update_todo(
    Path(id): Path<String>,
    prisma: Prisma,
    Json(data): Json<TodoQuery>,
) -> impl IntoResponse {
    let res = prisma
        .todo()
        .update(
            todo::id::equals(id),
            vec![
                todo::title::set(data.title),
                todo::completed::set(data.completed.unwrap_or_else(|| false)),
            ],
        )
        .exec()
        .await
        .unwrap();
    Json(res)
}

async fn delete_todo(Path(id): Path<String>, prisma: Prisma) -> impl IntoResponse {
    let res = prisma
        .todo()
        .delete(todo::id::equals(id))
        .exec()
        .await
        .unwrap();
    Json(res)
}
