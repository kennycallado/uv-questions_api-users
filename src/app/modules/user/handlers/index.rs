// extern
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// app
use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::config::database::Db;

// module
use crate::app::modules::user::model::User;
use crate::app::modules::user::services::respository as user_repository;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Therapist {
    pub id: i32,
    pub users: Vec<User>,
}

#[get("/", rank = 1)]
pub async fn get_index_admin(db: Db, _admin: AdminClaims) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::get_all(&db).await;

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/", rank = 2)]
pub async fn get_index_coord(db: Db, coord: CoordClaims) -> Result<Json<Vec<Therapist>>, Status> {
    let therapists = user_repository::get_users_by_depend(&db, coord.0.user.id).await;

    if let Err(_) = therapists {
        return Err(Status::NotFound);
    }
    let therapists = therapists.unwrap();

    let mut therapist_list: Vec<Therapist> = Vec::new();
    for therapist in therapists {
        let users = user_repository::get_users_by_depend(&db, therapist.id).await;

        if let Err(_) = users {
            continue;
        }
        let users = users.unwrap();

        therapist_list.push(Therapist {
            id: therapist.id,
            users,
        });
    }

    Ok(Json(therapist_list))
}

#[get("/", rank = 3)]
pub async fn get_index_thera(db: Db, thera: TheraClaims) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::get_users_by_depend(&db, thera.0.user.id).await;

    if let Err(_) = users {
        return Err(Status::NotFound);
    }
    let users = users.unwrap();

    Ok(Json(users))
}

#[get("/", rank = 5)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}
