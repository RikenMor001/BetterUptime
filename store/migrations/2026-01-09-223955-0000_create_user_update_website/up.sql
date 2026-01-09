-- Your SQL goes here
ALTER TABLE "website" ADD COLUMN "user_id" TEXT NOT NULL;

--Alter table
ALTER TABLE "website_tick" ADD COLUMN "updated_at" TIMESTAMP(3) NOT NULL
DEFAULT CURRENT_TIMESTAMP;

-- Create user table
CREATE TABLE "user" {
    "id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL

    CONSTRAINT "user_pkey" PRIMARY KEY("id")
};

-- Add foreign key relationship between the user that is created now
-- and the website that already exists
ALTER TABLE "website" ADD CONSTRAINT "website_user_id_fkey" FOREIGN KEY
("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
