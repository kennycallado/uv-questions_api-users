// extern
use rocket::http::Status;
use rocket::serde::json::Json;

// app
use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::app::providers::guards::user::UserClaims;
use crate::config::database::Db;

// module
use crate::app::modules::user::model::User;
use crate::app::modules::user::services::respository as user_repository;

#[get("/<id>", rank = 1)]
pub async fn get_show_admin(db: Db, _admin: AdminClaims, id: i32) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    Ok(Json(user))
}

#[get("/<id>", rank = 2)]
pub async fn get_show_coord(db: Db, coord: CoordClaims, id: i32) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user.role_id {
        2 => {
            // The user is a coord so the coord should be the same
            if user.id != coord.0.user.id {
                return Err(Status::Unauthorized);
            }
        }
        3 => {
            // The user is a thera so validate that the thera depends on the coord
            if user.depends_on != coord.0.user.id {
                println!("The user doesn't depend on the coord");
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // Validate that the user depends on a therapist of the coord
            let therapist = user_repository::get_user_by_id(&db, user.depends_on).await;

            if therapist.is_err() {
                println!("The user depends on a therapist that doesn't exist");
                return Err(Status::NotFound);
            }
            let therapist = therapist.unwrap();

            if therapist.depends_on != coord.0.user.id {
                println!("The user does't depend on a therapist of the coord");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    Ok(Json(user))
}

#[get("/<id>", rank = 3)]
pub async fn get_show_thera(db: Db, thera: TheraClaims, id: i32) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user.role_id {
        3 => {
            // The user is a thera so the thera should be the same
            if user.id != thera.0.user.id {
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // Validate that the user depends on the therapist
            if user.depends_on != thera.0.user.id {
                println!("The user does't depend on a this therapist");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    Ok(Json(user))
}

#[get("/<id>", rank = 4)]
pub async fn get_show_claims(db: Db, user: UserClaims, id: i32) -> Result<Json<User>, Status> {
    if user.0.user.id != id {
        return Err(Status::Unauthorized);
    }

    let user = user_repository::get_user_by_id(&db, id).await;
    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    Ok(Json(user))
}

#[get("/<_id>", rank = 5)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}
