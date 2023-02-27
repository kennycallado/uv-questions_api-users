// extern
use rocket::http::Status;
use rocket::serde::json::Json;

// app
use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::config::database::Db;

// module
use crate::app::modules::user::model::{User, UserPut};
use crate::app::modules::user::services::respository as user_repository;

#[put("/<id>", data = "<user>", rank = 1)]
pub async fn put_update_admin(
    db: Db,
    admin: AdminClaims,
    user: Json<UserPut>,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user.into_inner();

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

    let user = user_repository::update_user(&db, user, id).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/<id>", data = "<user>", rank = 2)]
pub async fn put_update_coord(
    db: Db,
    coord: CoordClaims,
    user: Json<UserPut>,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user.into_inner();

    match user.role_id {
        // Coord can't update other coords
        // 2 => {
        //     // updating a coord
        //     // Validate that the coord is the same
        //     if coord.0.user.id != id {
        //         return Err(Status::Unauthorized);
        //     }
        // }
        3 => {
            // updating a therapist
            // Validate that the thera depends on the coord
            if user.depends_on != coord.0.user.id {
                println!("The new_user does't depend on the coord");
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // updating a therapist
            // Validate that the user depends on a thera of the coord
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

    let user = user_repository::update_user(&db, user, id).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/<id>", data = "<user>", rank = 3)]
pub async fn put_update_thera(
    db: Db,
    thera: TheraClaims,
    user: Json<UserPut>,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user.into_inner();

    // Therapist can't update users
    return Err(Status::Unauthorized);
    // match user.role_id {
    //     3 => {
    //         // Updating a therapist
    //         // Validate that the thera is the same
    //         if thera.0.user.id != id {
    //             return Err(Status::Unauthorized);
    //         }
    //     }
    //     4 => {
    //         // Updating a patient
    //         // Validate that the userdepends on the thera
    //         if user.depends_on != thera.0.user.id {
    //             println!("The user does't depend on the thera");
    //             return Err(Status::Unauthorized);
    //         }
    //     }
    //     _ => return Err(Status::Unauthorized),
    // }

    // let user = user_repository::update_user(&db, user, id).await;

    // match user {
    //     Ok(user) => Ok(Json(user)),
    //     Err(_) => Err(Status::InternalServerError),
    // }
}

#[put("/", data = "<_user>", rank = 5)]
pub async fn put_update_none(_user: Json<UserPut>) -> Status {
    Status::Unauthorized
}
