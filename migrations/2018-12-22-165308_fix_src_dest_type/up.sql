DROP TABLE posts;

CREATE TABLE posts (
        id SERIAL PRIMARY KEY,
        src INTEGER NOT NULL REFERENCES accounts(id),
        dest INTEGER NOT NULL REFERENCES accounts(id),
        privacy INTEGER NOT NULL,
        content_warning VARCHAR,
        text VARCHAR,
        image_data JSON
)
