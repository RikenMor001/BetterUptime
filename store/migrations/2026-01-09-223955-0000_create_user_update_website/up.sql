-- Your SQL goes here

-- Add user_id to website (safe even if website already has rows)
ALTER TABLE "website" ADD COLUMN "user_id" TEXT;

-- Add updated_at to website_tick
ALTER TABLE "website_tick"
ADD COLUMN "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- Create user table
CREATE TABLE "user" (
    "id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);

-- Add foreign key relationship
ALTER TABLE "website"
ADD CONSTRAINT "website_user_id_fkey"
FOREIGN KEY ("user_id") REFERENCES "user"("id")
ON DELETE RESTRICT ON UPDATE CASCADE;
