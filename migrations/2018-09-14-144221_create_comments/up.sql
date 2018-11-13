CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),
    post_id SERIAL REFERENCES posts(id),
    body TEXT NOT NULL,
    creation_date TIMESTAMP WITH TIME ZONE NOT NULL
);
