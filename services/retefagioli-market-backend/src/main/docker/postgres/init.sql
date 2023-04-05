CREATE DATABASE "retefagioli-market";

\c retefagioli-market

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
    id uuid PRIMARY KEY,
    name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    dob DATE,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    address VARCHAR
);