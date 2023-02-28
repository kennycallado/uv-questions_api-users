use rocket::http::Status;

use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::user::UserClaims;
use crate::config::database::Db;

use crate::app::modules::user::services::respository as user_repository;

#[patch("/<id>/fcm", data = "<fcm_token>", rank = 1)]
pub async fn patch_update_fcm_token_admin(
    db: Db,
    admin: AdminClaims,
    id: i32,
    fcm_token: String,
) -> Result<Status, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user.role_id {
        1 => {
            // updating an admin
            // Validate that the admin is the same
            if admin.0.user.id != id {
                return Err(Status::Unauthorized);
            }
        }
        _ => {}
    }
    let user = user_repository::update_fcm_token(&db, id, fcm_token).await;

    match user {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>/fcm", data = "<fcm_token>", rank = 4)]
pub async fn patch_update_fcm_token_user(
    db: Db,
    user: UserClaims,
    id: i32,
    fcm_token: String,
) -> Result<Status, Status> {
    // Validate that the claim is the same as the id
    if user.0.user.id != id {
        return Err(Status::Unauthorized);
    }
    let user = user_repository::get_user_by_id(&db, id).await;

    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user.role_id {
        4 => {
            // updating an user
            // Validate that the user is the same
            // Unnecessary, but just in case
            if user.id != id {
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }
    let user = user_repository::update_fcm_token(&db, id, fcm_token).await;

    match user {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<_id>/fcm", data = "<_fcm_token>", rank = 5)]
pub async fn patch_update_fcm_token_none(_id: i32, _fcm_token: String) -> Status {
    Status::Unauthorized
}
