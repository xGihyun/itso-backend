use axum::{extract::State, response::Result, Form, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    // id: i64,
    // created_at: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    school: String,
    coach_name: String,
    category: String,
}

pub async fn register(State(pool): State<PgPool>, Json(user): Json<User>) -> Result<Json<User>> {
    let query = "INSERT INTO users (first_name, middle_name, last_name, school, coach_name, category) VALUES ($1, $2, $3, $4, $5, $6)";

    // let parsed_created_at_date =
    //     sqlx::types::chrono::DateTime::parse_from_str(&user.created_at, "%Y-%m-%d %H:%M:%S %z")
    //         .expect("Date and time is invalid.");

    sqlx::query(query)
        // .bind(&user.id)
        // .bind(parsed_created_at_date)
        .bind(&user.first_name)
        .bind(&user.middle_name)
        .bind(&user.last_name)
        .bind(&user.school)
        .bind(&user.coach_name)
        .bind(&user.category)
        .execute(&pool)
        .await
        .expect("Failed to register user.");

    Ok(Json(user))
}
