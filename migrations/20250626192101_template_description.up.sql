-- Add up migration script here
ALTER TABLE templates
    ADD COLUMN description TEXT NOT NULL DEFAULT '';