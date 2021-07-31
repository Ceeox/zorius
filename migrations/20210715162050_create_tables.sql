-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  email VARCHAR(254) NOT NULL,
  password_hash TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  invitation_pending BOOLEAN NOT NULL DEFAULT TRUE,
  firstname VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS intern_merchandise (
  id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
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

-- CREATE TYPE role AS ENUM (
--     'Admin', 
--     'MerchandiseModerator', 
--     'RoleModerator', 
--     'WorkAccountModerator', 
--     'WorkReportModerator', 
--     'NoRole'
-- );

-- CREATE TABLE IF NOT EXISTS roles (
--     id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
--     user_id UUID NOT NULL,
--     roles role ARRAY DEFAULT 'NoRole'
-- );