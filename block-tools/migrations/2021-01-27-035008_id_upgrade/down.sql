ALTER TABLE blocks
ALTER COLUMN id TYPE INT;

ALTER TABLE properties
ALTER COLUMN id TYPE INT,
ALTER COLUMN parent_id TYPE INT,
ALTER COLUMN value_id TYPE INT;