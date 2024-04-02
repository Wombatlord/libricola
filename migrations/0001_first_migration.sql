CREATE TABLE IF NOT EXISTS text_types (
    text_type_id INT GENERATED ALWAYS AS IDENTITY,
    text_type VARCHAR(255),
    PRIMARY KEY (text_type_id)
);

CREATE TABLE IF NOT EXISTS authors (
    author_id INT GENERATED ALWAYS AS IDENTITY,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    PRIMARY KEY (author_id)
);

CREATE TABLE IF NOT EXISTS texts (
    text_id INT GENERATED ALWAYS AS IDENTITY,
    text_type_id INT,
    author_id INT,
    title VARCHAR(255),
    published INT,
    metadata JSON,
    PRIMARY KEY (text_id)
);