CREATE DATABASE Prateleira;

CREATE TABLE Author (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    avatar BYTEA
);

CREATE TABLE Publisher (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    site VARCHAR(255),
    email VARCHAR(255),
    avatar BYTEA
);

CREATE TABLE Gender (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE User (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    nickname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    birthData DATE,
    registrationData DATE NOT NULL DEFAULT CURRENT_DATE,
    avatar BYTEA
);

CREATE TABLE Book (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    subtitle VARCHAR(255),
    publisher SERIAL NOT NULL REFERENCES Publisher(id),
    series_collection INT,
    volume INT,
    edition INT,
    publication_year INT,
    pages INT,
    language VARCHAR(100),
    isbn VARCHAR(15),
    synopsis TEXT,
    cover BYTEA
);

CREATE TABLE BookAuthor(
    id SERIAL PRIMARY KEY,
    book_id SERIAL NOT NULL REFERENCES Book(id),
    author_id SERIAL NOT NULL REFERENCES Author(id)
);

CREATE TABLE BookGender(
    id SERIAL PRIMARY KEY,
    book_id SERIAL NOT NULL REFERENCES Book(id),
    gender_id SERIAL NOT NULL REFERENCES Gender(id)
);

CREATE TABLE ReadingStatus(
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
)

CREATE TABLE BookUser(
    id SERIAL PRIMARY KEY,
    book_id SERIAL NOT NULL REFERENCES Book(id),
    user_id SERIAL NOT NULL REFERENCES User(id),
    have BOOLEAN,
    desirable BOOLEAN,
    favorite BOOLEAN,
    reading_status SERIAL REFERENCES ReadingStatus(id),
    pages_read INT,
    evaluation INT,
    review TEXT,
    reading_end_date DATE
);

CREATE TABLE UserFriendRequest(
    id SERIAL PRIMARY KEY,
    requesting_user_id SERIAL NOT NULL REFERENCES User(id),
    user_requested_id SERIAL NOT NULL REFERENCES User(id),
    date_request DATE NOT NULL DEFAULT CURRENT_DATE
);

CREATE TABLE UserFriendship(
    id SERIAL PRIMARY KEY,
    requesting_user_id SERIAL NOT NULL REFERENCES User(id),
    user_requested_id SERIAL NOT NULL REFERENCES User(id),
    friendship_date DATE NOT NULL DEFAULT CURRENT_DATE
);

INSERT INTO ReadingStatus (name) VALUES ('Lido');
INSERT INTO ReadingStatus (name) VALUES ('Lendo');
INSERT INTO ReadingStatus (name) VALUES ('Quero ler');
INSERT INTO ReadingStatus (name) VALUES ('Relendo');
INSERT INTO ReadingStatus (name) VALUES ('Abandonei');