-- Add migration script here
ALTER TABLE intern_merchandises ADD COLUMN controller_id UUID REFERENCES users;