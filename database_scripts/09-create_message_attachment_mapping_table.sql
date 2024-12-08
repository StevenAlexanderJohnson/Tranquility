CREATE TABLE attachment_mapping (
    post_id INTEGER REFERENCES message(id),
    attachment_id INTEGER REFERENCES attachment(id)
);