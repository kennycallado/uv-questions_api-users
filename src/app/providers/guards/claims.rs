use rocket::request::{FromRequest, Outcome, Request};

use crate::app::providers::interfaces::claims::{Claims, ClaimsError};
use crate::app::providers::interfaces::token::Token;

#[async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ClaimsError;

    // This claims is created from the private cookie
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token: Token = match Token::from_cookie(request) {
            Some(token) => token,
            None => {
                return Outcome::Failure((
                    rocket::http::Status::BadRequest,
                    ClaimsError::MissingToken,
                ));
            }
        };

        let claims = match token.decode() {
            Ok(claims) => claims.claims,
            Err(e) => {
                println!("Error: {:?}", e);
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    ClaimsError::InvalidToken,
                ));
            }
        };

        Outcome::Success(claims)
    }
}
