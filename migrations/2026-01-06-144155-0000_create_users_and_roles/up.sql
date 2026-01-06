-- Your SQL goes here

-- Creating UUID EXTENSION
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Roles table
CREATE TABLE roles (
    role_id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL UNIQUE,
    about TEXT NOT NULL
);

-- Users table
CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    refresh_hash VARCHAR, -- Nullable for first login
    role_id INT NOT NULL,

    CONSTRAINT fk_role
        FOREIGN KEY(role_id)
        REFERENCES roles(role_id)
        ON DELETE RESTRICT
);

-- Add a basic roles
INSERT INTO roles (title, about) VALUES
('ADMIN', 'System administrator'),
('USER', 'A regular habit-watcher'),
('BLOCKED', 'Temporarily suspended user');

-- Add a first system administrator (please change your credentials after your first login)
INSERT INTO users (username, email, password_hash, role_id)
VALUES (
    'admin',
    'admin@widehabit.com',
    '$argon2id$v=19$m=19456,t=2,p=1$OMTyYjQt7W2Byxnof5rrmg$ukzW4DGWSPrtBE0dm4yfFXP03UzH/Z5iwqy4tK41ngM', -- chonky1975 argon2id hash
    1
);