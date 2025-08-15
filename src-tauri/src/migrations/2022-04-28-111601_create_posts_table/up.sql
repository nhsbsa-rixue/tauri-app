CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO posts (title, body, published) VALUES
('First Post', 'This is the body of the first post.', 1),
('Second Post', 'This is the body of the second post.', 0),
('Third Post', 'This is the body of the third post.', 1);