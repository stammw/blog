TRUNCATE TABLE posts, users;

INSERT INTO users VALUES
(1, 'user1', 'user1@email.test', 'DrlvHeS4M5/iIE37xDZTy7uzBiTKSmoNblzNH2PFRU4='),
(2, 'user2', 'user2@email.test', 'password2'),
(3, 'user3', 'user3@email.test', 'password3');

ALTER SEQUENCE users_id_seq RESTART WITH 4;

INSERT INTO posts
(id, user_id, title, slug, body, published, creation_date)
VALUES
(1, 1, 'title1', 'title1', '# body1', true, TIMESTAMP '2018-08-17 10:23:54'),
(2, 1, 'title2: The second post', 'title2-the-second-post', 'body2', true, TIMESTAMP '2018-08-20 10:23:54'),
(3, 1, 'title3', 'title3', '# body3', false, TIMESTAMP '2018-08-21 10:23:54'),
(45, 1, 'title45', 'title45', '# body45', false, TIMESTAMP '2018-08-19 10:23:54');

ALTER SEQUENCE posts_id_seq RESTART WITH 46;

-- SELECT * FROM users;
-- SELECT * FROM posts;
