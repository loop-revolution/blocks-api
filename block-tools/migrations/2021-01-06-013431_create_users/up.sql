CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	username VARCHAR(36) NOT NULL UNIQUE,
	localuname VARCHAR(36) NOT NULL UNIQUE,
	password VARCHAR NOT NULL
)