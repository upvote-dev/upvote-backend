CREATE TABLE "reviews"
(
    id         INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    reviewee   TEXT        NOT NULL,
    username   VARCHAR(50) NOT NULL REFERENCES users (username),
    vote       SMALLINT    NOT NULL DEFAULT 0,
    message    TEXT,
    photo_url  VARCHAR(2048),
    video_url  VARCHAR(2048),
    created_at TIMESTAMP   NOT NULL DEFAULT current_timestamp
);
CREATE UNIQUE INDEX "reviews_reviewee_username_idx" ON reviews (reviewee, username);
