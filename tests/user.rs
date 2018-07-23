extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::local::{LocalResponse, Client};
use rocket::http::{Status, ContentType};

use stammw_blog::auth::UserToken;

fn get<'r, F>(path: &str, login: bool, check: F)
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

fn post<'r, F>(path: &str, body: &str, login: bool, check: F)
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

#[test]
fn new_users_errors_are_displayed() {
    let body = "name=&email=fds&password=pword";
    post("/user/create", body, true, |res| {
        assert_eq!(res.status(), Status::raw(400));
        let content = res.body_string().unwrap();
        println!("{}", content);
        assert!(content.contains("Name shall not be empty"));
        assert!(content.contains("Email is not valid"));
        assert!(content.contains("Password should be at least 8 chars"));
    });
}

#[test]
fn new_success_when_user_logged_in() {
    get("/user/new", false, |res| assert_eq!(res.status(), Status::raw(401)));
}

#[test]
fn new_fails_when_not_logged_in_and_some_user_exist() {
    get("/user/new", false, |res| assert_eq!(res.status(), Status::raw(401)));
}

#[test]
fn create_fails_when_username_already_exists() {
    let body = "name=user1&email=exists@domain.com&password=password";
    post("/user/create", body, true, |res| {
        assert_eq!(res.status(), Status::raw(400));
        assert!(res.body_string().unwrap().contains("user name already exists"));
    });
}

#[test]
fn create_fails_when_email_already_exists() {
    let body = "name=user1_does_not_exists&email=user1@email.test&password=password";
    post("/user/create", body, true, |res| {
        assert_eq!(res.status(), Status::raw(400));
        assert!(res.body_string().unwrap().contains("email already exists"));
    });
}

#[test]
fn create_success_when_user_logged_in() {
    let body = "name=create_success_when_user_logged_in\
        &email=create_success_when_user_logged_in@dsfq.fqsd&password=password";
    post("/user/create", body, true, |res| {
        assert_eq!(res.status(), Status::Ok);
    });
}

// #[test]
// fn create_fails_when_not_logged_in_and_some_user_exist() {
//     let res = user::create(None, user_repo(), None);
//     assert!(res.is_err());
// }

// #[test]
// fn create_succes_when_not_logged_in_and_no_user_exist() {
//     let res = user::create(None, user_repo_empty(), None);
//     assert!(res.is_ok());
// }
