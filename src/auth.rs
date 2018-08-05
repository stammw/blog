use rocket::{Outcome, State};
use rocket::outcome::{IntoOutcome};
use rocket::request::{FromRequest, Request};
use rocket::http::{Cookie, Status};
use frank_jwt::{self, Algorithm};
use Secret;
use time::Duration;
use serde_json;

const COOKIE_NAME: &'static str = "user";

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub id: i32,
    pub name: String,
}

impl UserToken {
    pub fn to_jwt(self, secret: &String) -> String {
        let header = json!({});
        frank_jwt::encode(header, secret, &json!(self), Algorithm::HS256)
            .unwrap()
    }

    pub fn from_jwt(jwt: String, secret: &String) -> Option<UserToken> {
        match frank_jwt::decode(&jwt, secret, Algorithm::HS256) {
            Ok((_, payload)) => Some(serde_json::from_value(payload).unwrap()),
            Err(e) => {
                warn!("jwt {} failed {:?}", jwt, e);
                None
            },
        
        }
    }
    
    pub fn to_cookie<'a>(self, secret: &String) -> Cookie<'a> {
        Cookie::build(COOKIE_NAME, self.to_jwt(&secret))
            .max_age(Duration::days(1))
            // .secure(true) // TODO uncomment once TLS is on
            .finish()
    }
}

// TODO there must be a way to dedup the following guards
impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = &'r str;

    fn from_request(request: &'a Request<'r>) -> Outcome<UserToken, (Status, &'r str), ()> {
        let error = (Status::Unauthorized, "You must be logged to access this url");
        if let Outcome::Success(secret) = request.guard::<State<Secret>>(){
            request.cookies().get(COOKIE_NAME)
                .and_then(|cookie| UserToken::from_jwt(cookie.value().to_string(), &secret.0))
                .into_outcome(error)
        } else {
            Outcome::Failure(error)
        }
    }
}

pub struct ForwardUserToken(pub UserToken);

impl<'a, 'r> FromRequest<'a, 'r> for ForwardUserToken {
    type Error = &'r str;

    fn from_request(request: &'a Request<'r>) -> Outcome<ForwardUserToken, (Status, &'r str), ()> {
        if let Outcome::Success(secret) = request.guard::<State<Secret>>(){
        request.cookies().get(COOKIE_NAME)
            .and_then(|cookie| UserToken::from_jwt(cookie.value().to_string(), &secret.0))
            .map(|token| ForwardUserToken(token))
            .or_forward(())
        } else {
            Outcome::Forward(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SECRET: &'static str = "secret";
    
    #[test]
    fn encoded_can_be_decoded() {
        let jwt = UserToken { id: 1, name: "name12".to_string() }.to_jwt(&SECRET.to_string());
        let token = UserToken::from_jwt(jwt, &SECRET.to_string()).unwrap();
        assert!(token.id == 1);
        assert!(token.name == "name12".to_string());
    }


    #[test]
    fn from_jwt_fails_if_invalid() {
        let token = UserToken::from_jwt("not a real jwt".to_string(), &SECRET.to_string());
        assert!(token.is_none());
    }
    
    #[test]
    fn from_jwt_fails_if_empty() {
        let token = UserToken::from_jwt(String::new(), &SECRET.to_string());
        assert!(token.is_none());
    }
}
