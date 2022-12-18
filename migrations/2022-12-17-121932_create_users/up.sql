-- Your SQL goes here-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR(140) NOT NULL,
  email VARCHAR(140) NOT NULL,
  created_at TIMESTAMP NOT NULL
)