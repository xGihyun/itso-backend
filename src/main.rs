// #![allow(unused_imports)]

use anyhow::anyhow;
use axum::{
    // extract::{Query, State},
    http::{HeaderValue, Method},
    // response::{Response, Result},
    routing::{get, post},
    Router,
};
// use oauth2::{
//     basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
//     ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, TokenResponse,
//     TokenUrl,
// };
// use reqwest::header::AUTHORIZATION;
// use serde::{Deserialize, Serialize};
use shuttle_secrets::SecretStore;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod web;

use web::{excel, registration};

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let db_url = if let Some(db_url) = secret_store.get("DATABASE_URL") {
        db_url
    } else {
        return Err(anyhow!("DATABASE_URL not found.").into());
    };

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Can't connect to database.");

    println!("\nNow listening to Postgres...");

    // let oauth_client = oauth_client(&secret_store).expect("Failed to get OAuth client");

    let app = Router::new()
        // .route("/auth/login", post(auth::login))
        // .route("/auth/register", post(auth::register))
        // .route("/login/test", get(fetch_user_info))
        // .with_state(oauth_client)
        .route("/", get(hello_world))
        .route("/register", post(registration::register))
        .route("/download", get(excel::generate_excel))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("Server has started, listening on: {}", addr);

    Ok(app.into())
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

async fn hello_world() -> &'static str {
    "Hello, World!"
}
