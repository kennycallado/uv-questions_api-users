use super::handlers::create;
use super::handlers::index;
use super::handlers::patch;
use super::handlers::show;
use super::handlers::update;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        // index
        index::get_index_admin,
        index::get_index_coord,
        index::get_index_thera,
        index::get_index_none,
        // show
        show::get_show_admin,
        show::get_show_coord,
        show::get_show_thera,
        show::get_show_claims,
        show::get_show_none,
        // create
        create::post_create_admin,
        create::post_create_coord,
        create::post_create_thera,
        create::post_create_none,
        // update
        update::put_update_admin,
        update::put_update_coord,
        // update::put_update_thera,
        update::put_update_none,
        // patch
        patch::patch_update_fcm_token_admin,
        patch::patch_update_fcm_token_user,
        patch::patch_update_fcm_token_none,
    ]
}
