use super::*;
use rocket::http::Header;
use rocket::http::{Accept, ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn test_health() {
    let client = Client::tracked(app::server::rocket()).unwrap();
    let response = client.get("/health").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("OK".into()));
}

#[test]
fn test_user_get_index_fails() {
    let client = Client::tracked(app::server::rocket()).unwrap();
    let response = client
        .get("/user")
        .header(Accept::JSON)
        .header(Header::new("Authorization", "Bearer 1234567890"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_user_get_index_pass() {
    let client = Client::tracked(app::server::rocket()).unwrap();

    let bearer = client
        .get("/auth/bypass/11")
        .header(Accept::JSON)
        .dispatch()
        .into_string()
        .unwrap();
    let bearer = bearer.trim_matches('"');

    let response = client
        .get("/user")
        .header(Accept::JSON)
        .header(Header::new("Authorization", format!("Bearer {bearer}")))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_user_get_show_pass() {
    let client = Client::tracked(app::server::rocket()).unwrap();

    let bearer = client
        .get("/auth/bypass/11")
        .header(Accept::JSON)
        .dispatch()
        .into_string()
        .unwrap();
    let bearer = bearer.trim_matches('"');

    let response = client
        .get("/user/11")
        .header(Accept::JSON)
        .header(Header::new("Authorization", format!("Bearer {bearer}")))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_user_post_create_fails() {
    let client = Client::tracked(app::server::rocket()).unwrap();

    let bearer = client
        .get("/auth/bypass/11")
        .header(Accept::JSON)
        .dispatch()
        .into_string()
        .unwrap();
    let bearer = bearer.trim_matches('"');

    let response = client
        .post("/user")
        .header(Accept::JSON)
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {bearer}")))
        .body(format!(
            r#"{{"depends_on": 1, "role_id": 4, "user_token": "kldfjslkdsja", "active": false }}"#
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}
