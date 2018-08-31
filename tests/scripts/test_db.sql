TRUNCATE TABLE users;
INSERT INTO users VALUES
(1, 'user1', 'user1@email.test', 'DrlvHeS4M5/iIE37xDZTy7uzBiTKSmoNblzNH2PFRU4='),
(2, 'user2', 'user2@email.test', 'password2'),
(3, 'user3', 'user3@email.test', 'password3');

TRUNCATE TABLE posts;
INSERT INTO posts
(id, title, slug, body, published, creation_date)
VALUES
(1, 'title1', 'title1', '# body1', true, TIMESTAMP '2018-08-19 10:23:54'),
(2, 'title2: The second post', 'title2-the-second-post', 'body2', true, TIMESTAMP '2018-08-19 10:23:54'),
(3, 'title3', 'title3', '# body3', false, TIMESTAMP '2018-08-19 10:23:54'),
(45, 'title45', 'title45', '# body45', false, TIMESTAMP '2018-08-19 10:23:54');

SELECT * FROM users;
SELECT * FROM posts;
