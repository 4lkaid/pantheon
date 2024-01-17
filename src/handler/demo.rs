use crate::{
    common::{error::Error, validation::ValidatedJson},
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
    let mut con = redis::get()
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
    ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"insert into users (username) values ($1) returning id, username"#,
        payload.username
    )
    .fetch_one(database::get())
    .await
    .map_err(|err| Error::Sqlx(err))?;
    Ok(Json(user))
}
