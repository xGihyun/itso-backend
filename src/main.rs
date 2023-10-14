// #![allow(unused_imports)]

use axum::{
    http,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
// use shuttle_secrets::SecretStore;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod web;

use web::{email, excel, models, registration};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::postgres::PgPool::connect(&db_url).await?;

    println!("\nSuccessfully connect to Postgres.");

    let email_addr = std::env::var("EMAIL_ADDRESS")?;
    let email_pass = std::env::var("EMAIL_PASSWORD")?;

    let email_credentials = models::EmailCredentials {
        address: email_addr,
        password: email_pass,
    };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/register", post(registration::register))
        .route("/download", get(excel::generate_excel))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<http::HeaderValue>()?)
                .allow_methods([http::Method::GET, http::Method::POST]),
        )
        .with_state(pool)
        .route("/email", post(email::send_email))
        .with_state(email_credentials);

    // let port: u16 = std::env::var("PORT")
    //     .unwrap_or_else(|_| "8000".to_string())
    //     .parse()?;

    // let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()?;

    let addr = format!("0.0.0.0:{port}").parse()?;

    println!("Server has started, listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// #[shuttle_runtime::main]
// async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
//     let db_url = secret_store
//         .get("DATABASE_URL")
//         .ok_or_else(|| anyhow!("DATABASE_URL not found."))?;
//
//     let pool = sqlx::postgres::PgPool::connect(&db_url)
//         .await
//         .map_err(|_| anyhow!("Could not connect to database."))?;
//
//     println!("\nNow listening to Postgres...");
//
//     let email_addr = secret_store
//         .get("EMAIL_ADDRESS")
//         .ok_or_else(|| anyhow!("EMAIL_ADDRESS not found."))?;
//
//     let email_pass = secret_store
//         .get("EMAIL_PASSWORD")
//         .ok_or_else(|| anyhow!("EMAIL_ADDRESS not found."))?;
//
//     let email_credentials = models::EmailCredentials {
//         address: email_addr,
//         password: email_pass,
//     };
//
//     let app = Router::new()
//         .route("/", get(hello_world))
//         .route("/register", post(registration::register))
//         .route("/download", get(excel::generate_excel))
//         .layer(
//             CorsLayer::new()
//                 .allow_origin("*".parse::<http::HeaderValue>().unwrap())
//                 .allow_methods([http::Method::GET, http::Method::POST]),
//         )
//         .with_state(pool)
//         .route("/email", post(email::send_email))
//         .with_state(email_credentials);
//
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
//
//     println!("Server has started, listening on: {}", addr);
//
//     Ok(app.into())
// }

async fn hello_world() -> http::StatusCode {
    println!("Hello, World!");

    http::StatusCode::OK
}

// fn oauth_client(secret_store: &SecretStore) -> Result<BasicClient, anyhow::Error> {
//     let google_client_id = ClientId::new(
//         if let Some(google_client_id) = secret_store.get("GOOGLE_CLIENT_ID") {
//             google_client_id
//         } else {
//             return Err(anyhow!("GOOGLE_CLIENT_ID not found."));
//         },
//     );
//
//     let google_client_secret = ClientSecret::new(
//         if let Some(google_client_secret) = secret_store.get("GOOGLE_CLIENT_ID") {
//             google_client_secret
//         } else {
//             return Err(anyhow!("GOOGLE_CLIENT_SECRET not found."));
//         },
//     );
//
//     let redirect_url = secret_store
//         .get("REDIRECT_URL")
//         .unwrap_or_else(|| "http://localhost:5173".to_string());
//     let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
//         .expect("Invalid authorization endpoint URL");
//     let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
//         .expect("Invalid token endpoint URL");
//
//     println!("Redirect URL: {redirect_url}");
//
//     let client = BasicClient::new(
//         google_client_id,
//         Some(google_client_secret),
//         auth_url,
//         Some(token_url),
//     )
//     .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"))
//     .set_revocation_uri(
//         RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
//             .expect("Invalid revocation endpoint URL"),
//     );
//
//     Ok(client)
// }
//
// async fn google_auth(State(client): State<BasicClient>) -> String {
//     let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
//     let (auth_url, _csrf_state) = &client
//         .authorize_url(CsrfToken::new_random)
//         .add_scope(Scope::new(
//             "https://www.googleapis.com/auth/userinfo.email".to_string(),
//         ))
//         .add_scope(Scope::new(
//             "https://www.googleapis.com/auth/userinfo.profile".to_string(),
//         ))
//         .set_pkce_challenge(pkce_code_challenge)
//         .url();
//
//     println!("Redirect URI: {:?}\n", client);
//     println!("Auth URL: {}", auth_url.as_ref());
//
//     auth_url.to_string()
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     id: String,
//     email: String,
//     name: String,
// }
//
// async fn fetch_user_info(
//     Query(query): Query<AuthRequest>,
//     State(client): State<BasicClient>,
// ) -> Result<String> {
//     let token = client
//         .exchange_code(AuthorizationCode::new(query.code.clone()))
//         .request_async(async_http_client)
//         .await
//         .context("failed in sending request request to authorization server")
//         .unwrap();
//
//     let client = reqwest::Client::new();
//
//     let user_data: User = client
//         .get("https://www.googleapis.com/oauth2/v1/userinfo")
//         .bearer_auth(token.access_token().secret())
//         .send()
//         .await
//         .context("failed in sending request to target Url")
//         .unwrap()
//         .json()
//         .await
//         .context("failed to deserialize response as JSON")
//         .unwrap();
//
//     println!("{:?}", user_data);
//
//     // Now `user_data` contains the user's ID, email, and name.
//     // You can use these values as needed.
//
//     Ok("hello".to_string())
// }
//
// #[derive(Debug, Deserialize)]
// struct AuthRequest {
//     code: String,
//     state: String,
// }

// async fn fetch_user_info(
//     Query(query): Query<AuthRequest>,
//     State(client): State<BasicClient>,
// ) -> String {
//     // Get an auth token
//     let token = client
//         .exchange_code(AuthorizationCode::new(query.code.clone()))
//         .request_async(async_http_client)
//         .await
//         .context("failed in sending request request to authorization server")?;
//
//     // Fetch user data from discord
//     let client = reqwest::Client::new();
//     let user_data = client
//         // https://discord.com/developers/docs/resources/user#get-current-user
//         .get("https://discordapp.com/api/users/@me")
//         .bearer_auth(token.access_token().secret())
//         .send()
//         .await
//         .context("failed in sending request to target Url")?
//         .json()
//         .await
//         .context("failed to deserialize response as JSON")?;
//
//     println!("{user_data:?}");
//
//     "Hello".to_string()
// }
