-- Add migration script here
-- extenstions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- triggers
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- user defined types/enums
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role') THEN
        CREATE TYPE role AS ENUM (
            'admin',
            'merchandise_moderator',
            'role_moderator',
            'work_account_moderator',
            'work_report_moderator',
            'no_role'
        );
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'my_type') THEN
        CREATE TYPE intern_merchandise_status AS ENUM (
            'ordered',
            'delivered',
            'stored',
            'used'
        );
    END IF;
END$$;



-- tables
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    email VARCHAR(254) NOT NULL,
    password_hash TEXT NOT NULL,
    invitation_pending BOOLEAN NOT NULL DEFAULT TRUE,
    firstname VARCHAR(255),
    lastname VARCHAR(255),
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


CREATE TABLE IF NOT EXISTS intern_merchandises (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    merchandise_id BIGINT,
    orderer_id UUID NOT NULL REFERENCES users,
    project_leader_id UUID NOT NULL REFERENCES users,
    purchased_on TIMESTAMP WITH TIME ZONE NOT NULL,
    count BIGINT NOT NULL,
    cost numeric(15,6) NOT NULL,
    status intern_merchandise_status NOT NULL,
    merchandise_name TEXT NOT NULL,
    use_case TEXT,
    location TEXT,
    article_number TEXT NOT NULL,
    shop TEXT NOT NULL,
    serial_number TEXT,
    arrived_on TIMESTAMP WITH TIME ZONE,
    url TEXT,
    postage numeric(15,6),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON intern_merchandises
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


CREATE TABLE IF NOT EXISTS customers (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    identifier TEXT NOT NULL,
    note TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON customers
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers,
    name TEXT NOT NULL,
    note TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON projects
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


CREATE TABLE IF NOT EXISTS work_reports (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL REFERENCES users,
    customer_id UUID NOT NULL REFERENCES customers,
    project_id UUID REFERENCES projects,
    description TEXT NOT NULL,
    invoiced BOOLEAN NOT NULL DEFAULT FALSE,
    report_started TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    report_ended TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON work_reports
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();