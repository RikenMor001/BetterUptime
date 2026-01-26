-- Create user table FIRST (must exist before FK is added)
CREATE TABLE "user" (
    id        TEXT NOT NULL,
    username  TEXT NOT NULL,
    password  TEXT NOT NULL,
    CONSTRAINT user_pkey PRIMARY KEY (id)
);

-- Making username unique
ALTER TABLE "user"
ADD CONSTRAINT user_username_unique UNIQUE(username);

-- Add user_id to website
ALTER TABLE website
ADD COLUMN user_id TEXT;

-- Add foreign key relationship (website.user_id -> user.id)
ALTER TABLE website
ADD CONSTRAINT website_user_id_fkey
FOREIGN KEY (user_id) REFERENCES "user"(id)
ON DELETE RESTRICT ON UPDATE CASCADE;

-- Add updated_at to website_tick
ALTER TABLE website_tick
ADD COLUMN updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP;
