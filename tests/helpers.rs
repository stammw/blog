extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
pub mod helpers{
    macro_rules! dispatch {
        ($method:expr, $path:expr, $test_fn:expr) => ({
            let client = Client::new(stammw_blog::rocket()).unwrap();
            $test_fn(&client, client.req($method, $path).dispatch());
        })
    }

    macro_rules! dispatch_user_post {
        ($path:expr, $data:expr, $test_fn:expr) => ({
            let client = Client::new(stammw_blog::rocket()).unwrap();
            $test_fn(&client, client.post($path)
                     .header(ContentType::Form)
                     .private_cookie(UserCookie::create(1, "test_user"))
                     .body(&$data)
                     .dispatch());
        })
    }

}
