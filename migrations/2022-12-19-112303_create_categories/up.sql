-- Your SQL goes here
CREATE TABLE categories (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR(140) NOT NULL,
  created_at TIMESTAMP NOT NULL
)