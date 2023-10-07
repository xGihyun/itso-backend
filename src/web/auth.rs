use axum::{extract::State, response::Result, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{PgPool, Row};

#[derive(Debug, Deserialize)]
pub struct UserAuth {
    email: String,
    password: String,
}

pub async fn login(State(pool): State<PgPool>, Json(user): Json<UserAuth>) -> Result<Json<Value>> {
    let q = "SELECT * FROM users WHERE email = ($1) AND password = ($2)";
    let query = sqlx::query(q);

    let _row = query
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&(pool))
        .await
        .expect("Invalid user.");

    println!("Welcome back, {}!", user.email);

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

enum Role {
    User,
    Admin,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
    role: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(new_user): Json<User>,
) -> Result<Json<Value>> {
    let q = "INSERT INTO users (email, password, role) VALUES ($1, $2, $3)";
    let query = sqlx::query(q);

    let _row = query
        .bind(&new_user.email)
        .bind(&new_user.password)
        .fetch_one(&(pool))
        .await
        .expect("Invalid user.");

    println!("Welcome, {}!", new_user.email);

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

// #[derive(Debug, Deserialize)]
// pub struct LogOut {
//     user_id: String,
// }
//
// pub async fn logout(State(pool): State<PgPool>, Form(logout): Form<LogOut>) -> Result<Json<Value>> {
//     let q = "UPDATE judges SET is_active = FALSE WHERE id = ($1)";
//     let query = sqlx::query(q);
//
//     query
//         .bind(&logout.user_id)
//         .execute(&(pool))
//         .await
//         .expect("Failed to update is_active to FALSE.");
//
//     println!("Goodbye!");
//
//     let body = Json(json!({
//         "result": {
//             "success": true
//         }
//     }));
//
//     Ok(body)
// }
