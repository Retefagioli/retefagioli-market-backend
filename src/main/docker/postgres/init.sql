CREATE DATABASE "retefagioli-market";

\c retefagioli-market

CREATE TABLE Utenti(
    id INT PRIMARY KEY,
    nome TEXT NOT NULL,
    cognome TEXT NOT NULL
);