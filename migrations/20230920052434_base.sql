CREATE TABLE IF NOT EXISTS "users"
(
    id         SERIAL,
    first_name VARCHAR(255) NOT NULL,
    last_name  VARCHAR(255) NOT NULL,
    user_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(255) NOT NULL,
    password   TEXT         NOT NULL,
    phone      VARCHAR(10)  NOT NULL,
    type       VARCHAR(255) NOT NULL, -- [admin, associate]
    state      VARCHAR(255) NOT NULL,
    country    VARCHAR(255) NOT NULL,
    -- add this line back after data import REFERENCES users(code) ON DELETE RESTRICT
    status     VARCHAR(255) NOT NULL DEFAULT 'Active',
    photo      VARCHAR(255),
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);