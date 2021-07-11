-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    username TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    invitation_pending BOOLEAN NOT NULL DEFAULT TRUE,
    avatar_url TEXT,
    firstname TEXT,
    lastname TEXT,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);

-- CREATE TABLE IF NOT EXISTS intern_merchandise
-- (
--     id UUID PRIMARY KEY,
--     merchandise_id: Option<i32>,
--     orderer: User,
--     project_leader: Option<User>,
--     purchased_on: DateTime<Utc>,
--     count: i32,
--     merchandise_name: String,
--     use_case: Option<String>,
--     location: Option<String>,
--     article_number: Option<String>,
--     shop: Option<String>,
--     cost: f64,
--     serial_number: Option<Vec<String>>,
--     arived_on: Option<DateTime<Utc>>,
--     status: InternMerchandiseStatus,
--     url: Option<String>,
--     postage: Option<f64>,
--     invoice_number: Option<i32>,
--     created_date: DateTime<Utc>,
--     updated_date: DateTime<Utc>,
-- );