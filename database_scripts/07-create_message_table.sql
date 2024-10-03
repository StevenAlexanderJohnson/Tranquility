CREATE TABLE (
    id SERIAL PRIMARY KEY,
    channel_id INTEGER REFERENCES channel(id),
    author_id INTEGER REFERENCES auth(id),
    content TEXT,
    created_date TIMESTAMPTZ DEFAULT (NOW() at TIME ZONE 'utc'),
    updated_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc')
);