use axum::{extract::State, response::Result, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct Participant {
    first_name: String,
    middle_name: String,
    last_name: String,
    school: String,
    coach_name: String,
    category: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(users): Json<Vec<Participant>>,
) -> Result<Json<Vec<Participant>>> {
    let query = "INSERT INTO participants (first_name, middle_name, last_name, school, coach_name, category) VALUES ($1, $2, $3, $4, $5, $6)";

    let mut transaction = pool.begin().await.expect("Failed to start transaction.");

    for user in users.iter() {
        sqlx::query(query)
            .bind(&user.first_name)
            .bind(&user.middle_name)
            .bind(&user.last_name)
            .bind(&user.school)
            .bind(&user.coach_name)
            .bind(&user.category)
            .execute(&mut *transaction)
            .await
            .expect("Failed to register user.");
    }

    transaction
        .commit()
        .await
        .expect("Failed to commit transaction.");

    Ok(Json(users))
}
