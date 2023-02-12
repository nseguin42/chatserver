CREATE TABLE logs
(
    id INTEGER GENERATED ALWAYS AS IDENTITY,
    timestamp TIMESTAMP DEFAULT NOW() NOT NULL,
    message TEXT,
    channel VARCHAR(25),
    chat_message_id INTEGER
        CONSTRAINT logs_chat_messages_id_fk
            REFERENCES chat_messages
);

