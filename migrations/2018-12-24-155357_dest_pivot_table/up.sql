CREATE TABLE posts_dests (
	id SERIAL PRIMARY KEY,
	post_id INTEGER REFERENCES posts(id) NOT NULL,
	dest_id INTEGER REFERENCES accounts(id) NOT NULL
);

CREATE INDEX ON posts_dests (post_id);
CREATE INDEX ON posts_dests (post_id, dest_id);
