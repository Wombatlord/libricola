CREATE TABLE IF NOT EXISTS text_types (
    text_type_id SERIAL NOT NULL,
    text_type VARCHAR(255),
    PRIMARY KEY (text_type_id)
);

CREATE TABLE IF NOT EXISTS authors (
    author_id SERIAL NOT NULL,
    author_name VARCHAR(255),
    PRIMARY KEY (author_id)
);

CREATE TABLE IF NOT EXISTS texts (
    text_id SERIAL NOT NULL,
    text_type_id INT,
    author_id INT,
    title VARCHAR(255),
    published INT,
    metadata JSON,
    PRIMARY KEY (text_id)
);