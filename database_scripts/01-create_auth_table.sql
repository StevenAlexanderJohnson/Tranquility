CREATE TABLE auth (
    id SERIAL PRIMARY KEY,
    username text,
    password text,
    email text,
    refresh_token text DEFAULT md5(random()::text),
    created_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    updated_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc')
)