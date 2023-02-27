use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::app::providers::interfaces::claims::Claims;
use crate::app::providers::interfaces::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AdminClaims(pub Claims);

#[async_trait]
impl<'r> FromRequest<'r> for AdminClaims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token: Token = match Token::from_header(request) {
            Some(token) => token,
            // None => return Outcome::Failure((rocket::http::Status::BadRequest, ())),
            None => return Outcome::Forward(()),
        };

        let claims = match token.decode() {
            Ok(claims) => claims.claims,
            Err(e) => {
                println!("AdminClaims: Error while decoding token");
                println!("Error: {:?}", e);
                return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
            }
        };

        if claims.user.role.name != "admin" {
            return Outcome::Forward(());
            // return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }

        Outcome::Success(AdminClaims(claims))
    }
}
