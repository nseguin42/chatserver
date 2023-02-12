CREATE TABLE chat_messages
(
    id INTEGER GENERATED ALWAYS AS IDENTITY
        CONSTRAINT chat_messages_pk
            PRIMARY KEY,
    username VARCHAR(25) NOT NULL
        CONSTRAINT chat_messages_users_name_fk
            REFERENCES users (name),
    channel VARCHAR(25) NOT NULL
        CONSTRAINT chat_messages_users_name_fk2
            REFERENCES users (name),
    timestamp TIMESTAMP NOT NULL,
    text VARCHAR(500) NOT NULL
);

