use crate::{
    common::{error::Error, validation::ValidatedJson},
    AppResult, AppState,
};
use axum::{extract::State, Json};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

pub async fn root(State(state): State<Arc<AppState>>) -> AppResult<String> {
    let mut con = state
        .redis
        .get_tokio_connection()
        .await
        .map_err(|err| Error::Redis(err))?;
    let _: () = con
        .set_ex("greeting", "Hello, Pantheon!", 10)
        .await
        .map_err(|err| Error::Redis(err))?;
    let result: String = con.get("greeting").await.map_err(|err| Error::Redis(err))?;
    Ok(result)
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"insert into users (username) values ($1) returning id, username"#,
        payload.username
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| Error::Sqlx(err))?;
    Ok(Json(user))
}
