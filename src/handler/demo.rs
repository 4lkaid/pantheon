use crate::{
    common::validation::ValidatedJson,
    config::{database, redis},
    AppResult,
};
use ::redis::AsyncCommands;
use axum::extract::Json;
use serde::{Deserialize, Serialize};
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

pub async fn root() -> AppResult<String> {
    let mut con = redis::conn().await?;
    let _: () = con.set_ex("greeting", "Hello, Pantheon!", 10).await?;
    let result: String = con.get("greeting").await?;
    Ok(result)
}

pub async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"insert into users (username) values ($1) returning id, username"#,
        payload.username
    )
    .fetch_one(database::conn())
    .await?;
    Ok(Json(user))
}
