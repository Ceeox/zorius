-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL,
    password_hash CHARACTER(254) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    invitation_pending BOOLEAN NOT NULL DEFAULT TRUE,
    firstname CHARACTER(255),
    lastname CHARACTER(255),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS intern_merchandise (
    id UUID PRIMARY KEY,
    merchandise_id INTEGER,
    orderer UUID,
    project_leader UUID,
    purchased_on TIMESTAMP WITH TIME ZONE NOT NULL,
    count INTEGER,
    merchandise_name TEXT NOT NULL,
    use_case TEXT,
    location TEXT,
    article_number TEXT,
    shop TEXT,
    cost NUMERIC NOT NULL,
    serial_number TEXT,
    arrived_on TIMESTAMP WITH TIME ZONE,
    status TEXT NOT NULL,
    url TEXT,
    postage NUMERIC,
    invoice_number NUMERIC,
    created_date TIMESTAMP WITH TIME ZONE,
    updated_date TIMESTAMP WITH TIME ZONE
);