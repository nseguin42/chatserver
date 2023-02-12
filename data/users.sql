CREATE TABLE users
(
    name VARCHAR(25) NOT NULL
        CONSTRAINT "Users_pk"
            PRIMARY KEY
);

ALTER TABLE users
    OWNER TO chatserver;

