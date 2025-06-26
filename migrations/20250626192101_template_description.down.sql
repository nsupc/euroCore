-- Add down migration script here
ALTER TABLE templates
    DROP COLUMN description;