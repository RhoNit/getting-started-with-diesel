ALTER TABLE posts
    ALTER COLUMN is_published SET DEFAULT TRUE,
    DROP COLUMN likes