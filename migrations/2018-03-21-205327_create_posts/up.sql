CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  slug VARCHAR UNIQUE NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);
