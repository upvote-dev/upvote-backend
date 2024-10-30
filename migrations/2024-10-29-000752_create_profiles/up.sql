CREATE TABLE "profiles"
(
    alias             VARCHAR(25) PRIMARY KEY,
    username          VARCHAR(50)                NOT NULL REFERENCES users (username),
    rank              TEXT                       NOT NULL DEFAULT 'beginner',
    coins             INTEGER CHECK (coins >= 0) NOT NULL DEFAULT 0,
    profile_image_url VARCHAR(2048),
    created_at        TIMESTAMP                  NOT NULL DEFAULT current_timestamp
);
CREATE UNIQUE INDEX "profiles_alias_idx" ON profiles (alias, username);
