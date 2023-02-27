use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::app::providers::interfaces::claims::{Claims, ClaimsError};
use crate::app::providers::interfaces::token::Token;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RobotClaims(pub Claims);

#[async_trait]
impl<'r> FromRequest<'r> for RobotClaims {
    type Error = ClaimsError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token: Token = match Token::from_header(request) {
            Some(token) => token,
            None => return Outcome::Forward(()),
        };

        let claims = match token.decode() {
            Ok(claims) => claims.claims,
            Err(e) => {
                println!("RobotClaims: Error while decoding token");
                println!("Error: {:?}", e);
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    ClaimsError::InvalidToken,
                ));
            }
        };

        if claims.user.role.name != "robot" {
            return Outcome::Forward(());
        }

        Outcome::Success(RobotClaims(claims))
    }
}
