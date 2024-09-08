CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    timestamp TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);
