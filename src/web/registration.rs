use axum::{extract::State, http, response::Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct Participant {
    category: String,
    school: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    coach_name: String,
    coach_email: String,
    coach_contact_number: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    axum::Json(users): axum::Json<Vec<Participant>>,
) -> Result<(http::StatusCode, axum::Json<Vec<Participant>>), http::StatusCode> {
    let query = "INSERT INTO participants (category, school, first_name, middle_name, last_name, coach_name, coach_email, coach_contact_number) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

    let start_transaction = pool.begin().await;

    match start_transaction {
        Ok(mut transaction) => {
            for user in users.iter() {
                let res = sqlx::query(query)
                    .bind(&user.category)
                    .bind(&user.school)
                    .bind(&user.first_name)
                    .bind(&user.middle_name)
                    .bind(&user.last_name)
                    .bind(&user.coach_name)
                    .bind(&user.coach_email)
                    .bind(&user.coach_contact_number)
                    .execute(&mut *transaction)
                    .await;

                if let Err(err) = res {
                    eprintln!("Failed to insert user: {}", err);

                    if let Err(err) = transaction.rollback().await {
                        eprintln!("Failed to rollback transaction: {}", err);
                    }

                    return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            }

            if let Err(err) = transaction.commit().await {
                eprintln!("Failed to commit transaction: {}", err);
                return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
            }

            return Ok((http::StatusCode::CREATED, axum::Json(users)));
        }
        Err(err) => {
            eprintln!("Failed to start transaction: {}", err);
            return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
