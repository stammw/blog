use frank_jwt::{self, Algorithm};
use serde_json;

static SECRET: &'static str = "todo_get_secret_from_conf";

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
            Err(_) => None,
        }
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
