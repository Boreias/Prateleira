CREATE DATABASE Prateleira;

CREATE TABLE author (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    deleted_at TIMESTAMP
);

CREATE TABLE author_image (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_name VARCHAR(255) NOT NULL,
    image_path TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES author(id) ON DELETE CASCADE,
    deleted_at TIMESTAMP
);

CREATE TABLE publisher (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    site VARCHAR(255),
    email VARCHAR(255),
    deleted_at TIMESTAMP
);

CREATE TABLE publisher_image (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_name VARCHAR(255) NOT NULL,
    image_path TEXT NOT NULL,
    publisher_id UUID NOT NULL REFERENCES publisher(id) ON DELETE CASCADE,
    deleted_at TIMESTAMP
);

CREATE TABLE gender (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    deleted_at TIMESTAMP
);

CREATE TABLE book (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    subtitle VARCHAR(255),
    publisher_id UUID NOT NULL REFERENCES Publisher(id) ON DELETE CASCADE,
    series_collection INT,
    volume INT,
    edition INT,
    publication_year INT,
    pages INT,
    language VARCHAR(100),
    isbn VARCHAR(25) NOT NULL,
    synopsis TEXT,
    deleted_at TIMESTAMP
);

CREATE TABLE book_image (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_name VARCHAR(255) NOT NULL,
    image_path TEXT NOT NULL,
    book_id UUID NOT NULL REFERENCES book(id) ON DELETE CASCADE,
    deleted_at TIMESTAMP
);

CREATE TABLE book_author(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES Book(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES Author(id) ON DELETE CASCADE,
    deleted_at TIMESTAMP
);

CREATE TABLE book_gender(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES Book(id) ON DELETE CASCADE,
    gender_id UUID NOT NULL REFERENCES Gender(id) ON DELETE CASCADE,
    deleted_at TIMESTAMP
);

CREATE TABLE reading_status(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL
);

INSERT INTO reading_status (name) VALUES ('Lido');
INSERT INTO reading_status (name) VALUES ('Lendo');
INSERT INTO reading_status (name) VALUES ('Quero ler');
INSERT INTO reading_status (name) VALUES ('Relendo');
INSERT INTO reading_status (name) VALUES ('Abandonei');

-- Tabelas de usuário

CREATE TABLE user_auth (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(30) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    salt VARCHAR(255) NOT NULL,
    country VARCHAR(2) NOT NULL,
    is_email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
);

CREATE TABLE user_profile (
    id UUID PRIMARY KEY REFERENCES user_auth(id),
    name VARCHAR(100) NOT NULL,
    bio TEXT,
    birth_date DATE
);

CREATE TABLE user_refresh_token (
    id UUID PRIMARY KEY REFERENCES user_auth(id),
    token TEXT NOT NULL,
    expire_at TIMESTAMP NOT NULL
);

CREATE TABLE user_email_verification (
    id UUID PRIMARY KEY REFERENCES user_auth(id),
    email VARCHAR(255) UNIQUE NOT NULL,
    token VARCHAR(255) NOT NULL
);

CREATE TABLE user_image (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_name VARCHAR(255) NOT NULL,
    image_path TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES user_auth(id) ON DELETE CASCADE
);


CREATE TABLE gender_created_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gender_id UUID NOT NULL REFERENCES gender(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE gender_updated_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gender_id UUID NOT NULL REFERENCES gender(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE gender_excluded_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gender_id UUID NOT NULL REFERENCES gender(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    excluded_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE publisher_created_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    publisher_id UUID NOT NULL REFERENCES publisher(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE publisher_updated_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    publisher_id UUID NOT NULL REFERENCES publisher(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE publisher_excluded_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    publisher_id UUID NOT NULL REFERENCES publisher(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    excluded_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE author_created_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES author(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE author_updated_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES author(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE author_excluded_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES author(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    excluded_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE book_created_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES book(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE book_updated_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES book(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE book_excluded_at (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES book(id),
    user_id UUID NOT NULL REFERENCES user_profile(id),
    excluded_at TIMESTAMP NOT NULL DEFAULT NOW()
);