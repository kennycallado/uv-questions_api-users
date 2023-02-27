// extern
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;

// app
use crate::app::providers::interfaces::claims::Claims;
use crate::config::database::Db;

// modules
use crate::app::modules::auth::services::helpers;
use crate::app::modules::user::services::respository as user_repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![auth_bypass, auth, login]
}

// WARNING: This is only for testing purposes
#[get("/bypass/<id>")]
pub async fn auth_bypass(db: Db, cookie: &CookieJar<'_>, id: i32) -> Result<Json<String>, Status> {
    let user = user_repository::get_user_expanded_by_id(&db, id).await;
    // let user = user_repository::get_user_by_id(&db, id).await;

    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    let mut claims: Claims = Claims::from(user);
    let token = claims.encode_for_refresh().unwrap();

    cookie.add_private(Cookie::new("refresh_token", token.clone()));

    Ok(Json(token))
}

#[get("/")]
pub async fn auth(db: Db, cookie: &CookieJar<'_>, claims: Claims) -> Result<Json<String>, Status> {
    // let user = user_repository::get_user_by_id(&db, claims.user.id).await;
    let user = user_repository::get_user_expanded_by_id(&db, claims.user.id).await;

    // The users does not exist
    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    // check the user_token is the same
    if user.user_token != claims.user.user_token {
        return Err(Status::Unauthorized);
    }

    match helpers::token_generator(&db, user).await {
        Ok((refresh_token, access_token)) => {
            cookie.add_private(Cookie::new("refresh_token", refresh_token.clone()));

            Ok(Json(access_token))
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[post("/login", data = "<token>")]
pub async fn login(db: Db, cookie: &CookieJar<'_>, token: String) -> Result<Json<String>, Status> {
    // Request the user_id from the profile api
    let response = helpers::profile_request(token).await;
    if let Err(e) = response {
        return Err(e);
    }
    let response = response.unwrap();

    let user = user_repository::get_user_expanded_by_id(&db, response).await;
    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match helpers::token_generator(&db, user).await {
        Ok((refresh_token, access_token)) => {
            cookie.add_private(Cookie::new("refresh_token", refresh_token.clone()));

            Ok(Json(access_token))
        }
        Err(e) => {
            return Err(e);
        }
    }
}
