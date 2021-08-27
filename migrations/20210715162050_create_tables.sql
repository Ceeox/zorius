-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    email VARCHAR(254) NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    invitation_pending BOOLEAN NOT NULL DEFAULT TRUE,
    firstname VARCHAR(255),
    lastname VARCHAR(255),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);

DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role') THEN
            CREATE TYPE role AS ENUM (
                'Admin',
                'MerchandiseModerator',
                'RoleModerator',
                'WorkAccountModerator',
                'WorkReportModerator',
                'NoRole'
                );
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    roles role ARRAY
);

DO $$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'intern_merchandise_status') THEN
            CREATE TYPE intern_merchandise_status AS ENUM (
                'Ordered',
                'Delivered',
                'RoleModerator',
                'Stored',
                'Used'
                );
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS intern_merchandise (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    merchandise_id BIGINT,
    orderer_id UUID NOT NULL,
    project_leader_id UUID NOT NULL,
    purchased_on TIMESTAMP WITH TIME ZONE NOT NULL,
    count BIGINT NOT NULL,
    cost numeric(15,6) NOT NULL,
    merch_status intern_merchandise_status NOT NULL,
    merchandise_name TEXT NOT NULL,
    use_case TEXT,
    location TEXT,
    article_number TEXT NOT NULL,
    shop TEXT NOT NULL,
    serial_number TEXT,
    arrived_on TIMESTAMP WITH TIME ZONE,
    url TEXT,
    postage numeric(15,6),
    invoice_number BIGINT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

