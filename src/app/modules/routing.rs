// use super::auth::controller as auth_controller;

// this file is needed to avoid public access to the modules
pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
        // .mount("/api/v1/auth", auth_controller::routes())
    })
}
