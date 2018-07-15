use rocket::Outcome;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Request};
use rocket::http::{Cookie, Status};
use frank_jwt::{self, Algorithm};
use time::Duration;
use serde_json;

static SECRET: &'static str = "todo_get_secret_from_conf";
const COOKIE_NAME: &'static str = "user";

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub id: i32,
    pub name: String,
}

impl UserToken {
    pub fn to_jwt(self) -> String {
        let header = json!({});
        frank_jwt::encode(header, &SECRET.to_string(), &json!(self), Algorithm::HS256)
            .unwrap()
    }

    pub fn from_jwt(jwt: String) -> Option<UserToken> {
        match frank_jwt::decode(&jwt, &SECRET.to_string(), Algorithm::HS256) {
            Ok((_, payload)) => Some(serde_json::from_value(payload).unwrap()),
            Err(e) => {
                warn!("jwt {} failed {:?}", jwt, e);
                None
            },
        
        }
    }
    
    pub fn to_cookie<'a>(self) -> Cookie<'a> {
        Cookie::build(COOKIE_NAME, self.to_jwt())
            .max_age(Duration::days(1))
            // .secure(true) // TODO uncomment once TLS is on
            .finish()
    }
}

pub struct ForwardUserToken(UserToken);

impl<'a, 'r> FromRequest<'a, 'r> for ForwardUserToken {
    type Error = &'r str;

    fn from_request(request: &'a Request<'r>) -> Outcome<ForwardUserToken, (Status, &'r str), ()> {
        request.cookies().get(COOKIE_NAME)
            .and_then(|cookie| UserToken::from_jwt(cookie.value().to_string()))
            .map(|token| ForwardUserToken(token))
            .or_forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = &'r str;

    fn from_request(request: &'a Request<'r>) -> Outcome<UserToken, (Status, &'r str), ()> {
        request.cookies().get(COOKIE_NAME)
            .and_then(|cookie| UserToken::from_jwt(cookie.value().to_string()))
            .into_outcome((Status::Unauthorized, "You must be logged to access this url"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn encoded_can_be_decoded() {
        let jwt = UserToken { id: 1, name: "name12".to_string() }.to_jwt();
        let token = UserToken::from_jwt(jwt).unwrap();
        assert!(token.id == 1);
        assert!(token.name == "name12".to_string());
    }


    #[test]
    fn from_jwt_fails_if_invalid() {
        let token = UserToken::from_jwt("not a real jwt".to_string());
        assert!(token.is_none());
    }
    
    #[test]
    fn from_jwt_fails_if_empty() {
        let token = UserToken::from_jwt(String::new());
        assert!(token.is_none());
    }
}
