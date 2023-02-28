use rocket::http::Status;
use rocket::serde::uuid::Uuid;

use crate::config::database::Db;

use crate::app::providers::interfaces::claims::{Claims, UserInClaims};
use crate::app::providers::interfaces::config_getter::ConfigGetter;

use crate::app::modules::user::model::UserExpanded;
use crate::app::modules::user::services::respository as user_repository;

pub async fn profile_request(token: String) -> Result<i32, Status> {
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    let profile_api_url = ConfigGetter::get_profile_url()
        .unwrap_or("http://localhost:8001/api/v1/profile".to_string())
        + "/token";

    let client = reqwest::Client::new();
    let res = client
        .post(&profile_api_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status() != 200 {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<i32>().await.unwrap())
        }
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    }
}

/// Ensures the user has user_token and returns a tuple of refresh_token and access_token
pub async fn token_generator(db: &Db, mut user: UserExpanded) -> Result<(String, String), Status> {
    if user.user_token.is_none() {
        return Err(Status::Unauthorized);
    }

    user.user_token = Some(Uuid::new_v4().to_string());

    let user_token = user.user_token.clone().unwrap();
    let user_token = user_repository::update_user_token(&db, user.id, user_token).await;

    match user_token {
        Ok(_) => {
            let mut claims: Claims = Claims::from(user);

            let refresh_token = claims.encode_for_refresh();
            if let Err(_) = refresh_token {
                return Err(Status::InternalServerError);
            }
            let refresh_token = refresh_token.unwrap();

            // encode_for_access removes claims.user.user_token
            let access_token = claims.encode_for_access();
            if let Err(_) = access_token {
                return Err(Status::InternalServerError);
            }
            let access_token = access_token.unwrap();

            Ok((refresh_token, access_token))
        }
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    }
}

pub async fn robot_token_generator() -> Result<String, Status> {
    let mut claims: Claims = Claims::from(UserInClaims::default());

    let access_token = claims.enconde_for_robot();
    if let Err(_) = access_token {
        return Err(Status::InternalServerError);
    }

    match access_token {
        Ok(access_token) => Ok(access_token),
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    }
}
