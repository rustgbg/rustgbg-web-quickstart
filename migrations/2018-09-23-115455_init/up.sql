-- Your SQL goes here

CREATE TABLE examples (
    id TEXT PRIMARY KEY NOT NULL,
    value1 TEXT,
    value2 INT
);

INSERT INTO examples (id, value1, value2) VALUES ("test1", "This is a test", 1);
INSERT INTO examples (id, value1, value2) VALUES ("test2", "Another test", 42);
INSERT INTO examples (id, value1, value2) VALUES ("test3", "This is a test", -124);