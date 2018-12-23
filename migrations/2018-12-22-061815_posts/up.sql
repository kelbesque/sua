CREATE TABLE posts (
	id SERIAL PRIMARY KEY,
	src VARCHAR NOT NULL,
	dest VARCHAR NOT NULL,
	privacy INTEGER NOT NULL,
	content_warning VARCHAR,
	text VARCHAR,
	image_data JSON
)
