-- Your SQL goes here
-- Create enum
CREATE TYPE website_status AS ENUM ('Up', 'Down', 'Unknown');

-- website table
CREATE TABLE website (
    id           TEXT        NOT NULL,
    url          TEXT        NOT NULL,
    time_added   TIMESTAMPTZ NOT NULL,
    CONSTRAINT website_pkey PRIMARY KEY (id)
);

-- region table
CREATE TABLE region (
    id    TEXT NOT NULL,
    name  TEXT NOT NULL,
    CONSTRAINT region_pkey PRIMARY KEY (id)
);

-- website_tick table
CREATE TABLE website_tick (
    id                TEXT          NOT NULL,
    website_id        TEXT          NOT NULL,
    region_id         TEXT          NOT NULL,
    response_time_ms  INTEGER       NOT NULL,
    status            website_status NOT NULL,
    CONSTRAINT website_tick_pkey PRIMARY KEY (id),
    CONSTRAINT website_tick_region_id_fkey
        FOREIGN KEY (region_id) REFERENCES region(id)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT website_tick_website_id_fkey
        FOREIGN KEY (website_id) REFERENCES website(id)
        ON DELETE RESTRICT ON UPDATE CASCADE
);