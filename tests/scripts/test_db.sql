TRUNCATE TABLE users;
INSERT INTO users VALUES
(1, 'user1', 'user1@email.test', 'DrlvHeS4M5/iIE37xDZTy7uzBiTKSmoNblzNH2PFRU4='),
(2, 'user2', 'user2@email.test', 'password2'),
(3, 'user3', 'user3@email.test', 'password3');

TRUNCATE TABLE posts;
INSERT INTO posts VALUES
(1, 'title1', 'body1', true),
(2, 'title2', 'body2', true),
(3, 'title3', '# body3', false);

SELECT * FROM users;
