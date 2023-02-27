// extern
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid};

// app
use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::config::database::Db;

// module
use crate::app::modules::user::model::{NewUser, User};
use crate::app::modules::user::services::respository as user_repository;

#[post("/", data = "<new_user>", rank = 1)]
pub async fn post_create_admin(
    db: Db,
    _admin: AdminClaims,
    new_user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    let user = user_repository::add_user(&db, new_user.into_inner()).await;
    if let Err(_) = user {
        return Err(Status::InternalServerError);
    }
    let mut user = user.unwrap();

    user.user_token = Some(Uuid::new_v4().to_string());

    let user_token = user.user_token.clone().unwrap();
    let user_token = user_repository::update_user_token(&db, user.id, user_token).await;

    match user_token {
        Ok(_) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<new_user>", rank = 2)]
pub async fn post_create_coord(
    db: Db,
    coord: CoordClaims,
    new_user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    let new_user = new_user.into_inner();

    match new_user.role_id {
        3 => {
            // Creating a therapist
            // Validate that the new_user depends on the coord
            if new_user.depends_on != coord.0.user.id {
                println!("The new_user does't depend on the coord");
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // Creating a patient
            // Validate that the new_user depends on a therapist of the coord
            let therapist = user_repository::get_user_by_id(&db, new_user.depends_on).await;

            if therapist.is_err() {
                println!("The new_user depends on a therapist that doesn't exist");
                return Err(Status::NotFound);
            }
            let therapist = therapist.unwrap();

            if therapist.depends_on != coord.0.user.id {
                println!("The new_user does't depend on a therapist of the coord");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    let user = user_repository::add_user(&db, new_user).await;
    if let Err(_) = user {
        return Err(Status::InternalServerError);
    }
    let mut user = user.unwrap();

    user.user_token = Some(Uuid::new_v4().to_string());

    let user_token = user.user_token.clone().unwrap();
    let user_token = user_repository::update_user_token(&db, user.id, user_token).await;

    match user_token {
        Ok(_) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<new_user>", rank = 3)]
pub async fn post_create_thera(
    db: Db,
    thera: TheraClaims,
    new_user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    let new_user = new_user.into_inner();

    match new_user.role_id {
        4 => {
            // Creating a patient
            // Validate that the new_user depends on the thera
            if new_user.depends_on != thera.0.user.id {
                println!("The new_user does't depend on the thera");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    let user = user_repository::add_user(&db, new_user).await;
    if let Err(_) = user {
        return Err(Status::InternalServerError);
    }
    let mut user = user.unwrap();

    user.user_token = Some(Uuid::new_v4().to_string());

    let user_token = user.user_token.clone().unwrap();
    let user_token = user_repository::update_user_token(&db, user.id, user_token).await;

    match user_token {
        Ok(_) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<_new_user>", rank = 5)]
pub async fn post_create_none(_new_user: Json<NewUser>) -> Status {
    Status::Unauthorized
}
