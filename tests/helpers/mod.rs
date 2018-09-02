use rocket::local::{LocalRequest, LocalResponse, Client};
use rocket::http::{ContentType};

use stammw_blog;
use stammw_blog::auth::UserToken;

#[allow(dead_code)]
pub fn post<'r, F>(path: &str, body: &str, login: bool, check: F)
where F: for<'c, 's> Fn(&'c mut LocalResponse<'s>) {
    let secret = String::from("test_secret");
    let rocket = stammw_blog::rocket();
    let client = Client::new(rocket).unwrap();
    let mut req = client.post(path).header(ContentType::Form);

    if login {
       req = req.cookie(UserToken { id: 1, name: "user1".to_string() }.to_cookie(&secret));
    }

    check(&mut req.body(body).dispatch());
}

#[allow(dead_code)]
pub fn post_req<'r, F>(path: &str, body: &str, login: bool, mut check: F)
where F: for<'c, 's> FnMut(LocalRequest<'s>) {
    let secret = String::from("test_secret");
    let rocket = stammw_blog::rocket();
    let client = Client::new(rocket).unwrap();
    let mut req = client.post(path).header(ContentType::Form);

    if login {
       req = req.cookie(UserToken { id: 1, name: "user1".to_string() }.to_cookie(&secret));
    }

    check(req.body(body));
}

#[allow(dead_code)]
pub fn get<'r, F>(path: &str, login: bool, check: F)
where F: for<'c, 's> Fn(&'c mut LocalResponse<'s>) {
    let secret = String::from("test_secret");
    let rocket = stammw_blog::rocket();
    let client = Client::new(rocket).unwrap();
    let mut req = client.get(path);

    if login {
       req = req.cookie(UserToken { id: 1, name: "user1".to_string() }.to_cookie(&secret));
    }

    check(&mut req.dispatch());
}
