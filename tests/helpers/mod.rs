
#[macro_export]
macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => ({
        let client = Client::new(stammw_blog::rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    })
}

#[macro_export]
macro_rules! dispatch_request {
    ($method:expr, $path:expr, $req_fn:expr, $test_fn:expr) => ({
        let client = Client::new(stammw_blog::rocket()).unwrap();
        let req = client.req($method, $path);
        $req_fn(&req);
        $test_fn(&client, req.dispatch());
    })
}

#[macro_export]
macro_rules! dispatch_post {
    ($path:expr, $data:expr, $repo:expr, $test_fn:expr) => ({
        let rocket = stammw_blog::rocket_stateless()
                 .manage($repo);
        let client = Client::new(rocket).unwrap();
        $test_fn(client.post($path)
                 .header(ContentType::Form)
                 .private_cookie(UserCookie::create(1, "test_user"))
                 .body(&$data)
                 .dispatch());
    })
}
