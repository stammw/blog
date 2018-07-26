extern crate serde_json;
extern crate stammw_blog;

use serde_json::value::to_value;
use stammw_blog::models::Post;

#[test]
fn post_can_be_serialied() {
    let post = Post {
        id: 1,
        slug: "Title".to_string(),
        title: "Title".to_string(),
        body: "Text".to_string(),
        published: true,
    };

    let serialized = serde_json::to_string(&post).unwrap();
    println!("serialized = {}", serialized);
    let value = to_value(post).unwrap();
    assert!(value.is_object());
}

#[test]
fn post_vec_can_be_serialied() {
    let posts = vec![
        Post {
            id: 1,
            slug: "Title".to_string(),
            title: "Title".to_string(),
            body: "Text".to_string(),
            published: true,
        },
        Post {
            id: 1,
            slug: "Title".to_string(),
            title: "Title".to_string(),
            body: "Text".to_string(),
            published: true,
        },
    ];

    let serialized = serde_json::to_string(&posts).unwrap();
    println!("serialized = {}", serialized);
    let value = to_value(posts).unwrap();
    assert!(value.is_array());
}
