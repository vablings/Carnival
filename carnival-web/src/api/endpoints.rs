use std::net::SocketAddr;

use easy_password::bcrypt::verify_password;
use static_str_ops::static_format;
use axum::{
    extract::{State, ConnectInfo},
    Json, response::Response, body::{Full, Bytes}
};
use http::{StatusCode, HeaderValue};
use crate::{
    api::payloads::{RegisterInput, LoginInput},
    CarnyState,
    db::services::user::{self}, HMAC_KEY
};
use crate::db::services::session_token as session;

pub async fn register(
    State(state): State<CarnyState>,
    Json(post_data): Json<RegisterInput>
)-> (StatusCode, String) {

    // NOTE(aalhendi): is this needed?
    let username: &str = &post_data.username;
    let password: &str = &post_data.password;
    let password_conf: &str = &post_data.password_conf;
    let battletag: &str = &post_data.battletag;

    if password != password_conf {
        return (StatusCode::BAD_REQUEST,
                "Passwords do not match".to_string());
    }

    if user::does_battletag_exist(battletag, &state.pool).await {
        return (StatusCode::BAD_REQUEST, "Battletag already exists".to_string())
    }

    if user::does_username_exist(username, &state.pool).await {
        return (StatusCode::BAD_REQUEST,
                "Username already exists".to_string());
    }

    match user::create_user(username, password, battletag, &state.pool).await {
        Ok(_) => (StatusCode::OK, "Created".to_string()),
        Err(e) => {
            eprintln!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating user.".to_string())
        }
    }
}

#[axum_macros::debug_handler]
pub async fn login(
    ConnectInfo(connection): ConnectInfo<SocketAddr>,
    State(state): State<CarnyState>,
    Json(post_data): Json<LoginInput>
) -> Response<Full<Bytes>> {

    let mut r: Response<Full<Bytes>> = Response::new(Full::from("nil"));

    let username: &str = &post_data.username;
    let password: &str = &post_data.password;
    let user_result = user::user_by_username(username, &state.pool).await;

    let user = match user_result {
        Ok(unwrapped_user) => unwrapped_user,
        // If there is no user by the username posted to us, error out.
        Err(_) =>  {
            *r.status_mut() = StatusCode::BAD_REQUEST;
            *r.body_mut() = Full::from("User does not exist");
            return r;
        }
    };

    if verify_password(password, &user.password, HMAC_KEY).unwrap() {
        // checks to see if the requesting user has a valid token already.
        let needs_token = session::token_by_user_id(user.id, &state.pool).await.is_none();
        // If they don't, create one. 
        if needs_token {
            let session_option = session::create(&connection, user.id, &state.pool).await;
            // This would suck.
            if session_option.is_none() {
                *r.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *r.body_mut() = Full::from("Could not create token");
                return r;
            }
        }

        // Validate that the token hasn't been stolen.
        // TODO: This shouldn't be happening here
        //      There's no reason to validate the session token on login.
        //      If someone knows a user's password, they know their password.
        //      They will still be assigned a token (as of now).
        //
        //      This *should* be tossed into middleware of some kind and then 
        //      invoked prior to execution on any endpoint route that is denoted
        //      to require authorization.
        //
        match session::validate(&connection, user.id, &state.pool).await {
            // If it hasn't, cool. Set the cookie and be done with it.
            Some(session) => {
                *r.status_mut() = StatusCode::OK;
                *r.body_mut() = Full::from("Nice");
                r.headers_mut().insert(
                    "set-cookie",
                    HeaderValue::from_str(static_format!("session={}", session)).unwrap()
                );
            },
            // If it has, we get very upset and tell the client that their actions are
            // "NOT_ACCEPTABLE".
            None => {
                *r.status_mut() = StatusCode::NOT_ACCEPTABLE;
                *r.body_mut() = Full::from("Fuck you?");
            }
        }
        return r;
    }
  
    // If we haven't dipped yet then this is the only remaining possibility.
    *r.status_mut() = StatusCode::BAD_REQUEST;
    *r.body_mut() = Full::from("Incorrect username or password");
    r
}