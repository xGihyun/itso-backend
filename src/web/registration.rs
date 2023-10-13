use axum::{extract::State, response::Result, Json};
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
    Json(users): Json<Vec<Participant>>,
) -> Result<Json<Vec<Participant>>> {
    let query = "INSERT INTO participants (category, school, first_name, middle_name, last_name, coach_name, coach_email, coach_contact_number) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

    let start_transaction = pool.begin().await;

    match start_transaction {
        Ok(mut transaction) => {
            for user in users.iter() {
                sqlx::query(query)
                    .bind(&user.category)
                    .bind(&user.school)
                    .bind(&user.first_name)
                    .bind(&user.middle_name)
                    .bind(&user.last_name)
                    .bind(&user.coach_name)
                    .bind(&user.coach_email)
                    .bind(&user.coach_contact_number)
                    .execute(&mut *transaction)
                    .await
                    .expect("Failed to register user.");
            }

            transaction
                .commit()
                .await
                .expect("Failed to commit transaction.");
        }
        Err(err) => eprintln!("Failed to start transaction: {err:?}"),
    }

    Ok(Json(users))
}
