-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--


CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    permission SMALLINT DEFAULT 0 NOT NULL,
    full_name VARCHAR NOT NULL,
    email VARCHAR(254) UNIQUE NOT NULL,
    "password" VARCHAR NOT NULL,
    job_title VARCHAR DEFAULT '' NOT NULL,
    profile_picture VARCHAR DEFAULT '' NOT NULL,
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('users');

--


CREATE TABLE coordinates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    address VARCHAR DEFAULT '' NOT NULL,
    telephone_no VARCHAR DEFAULT '' NOT NULL,
    fax VARCHAR DEFAULT '' NOT NULL,
    cellphone_no VARCHAR DEFAULT '' NOT NULL,
    email VARCHAR DEFAULT '' NOT NULL,
    company_name VARCHAR DEFAULT '' NOT NULL,
    company_number VARCHAR DEFAULT '' NOT NULL,
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('coordinates');

--


CREATE TABLE building_managers ( -- organization
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    full_name VARCHAR,
    profile_picture VARCHAR,
    coordinates_id UUID REFERENCES coordinates(id),
    linked_user_id UUID REFERENCES users(id),
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('building_managers');


--

CREATE TABLE building_owners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    full_name VARCHAR,
    is_manager BOOL,
    manager_id UUID REFERENCES building_managers(id),
    linked_user_id UUID REFERENCES users(id),
    coordinates_id UUID REFERENCES coordinates(id),
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('building_owners');


--

CREATE TABLE buildings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL REFERENCES building_owners(id),
    manager_id UUID NOT NULL REFERENCES building_managers(id),
    respondant_id UUID NOT NULL REFERENCES users(id),
    "name" VARCHAR NOT NULL,
    "address" VARCHAR NOT NULL,
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('buildings');

--

CREATE TABLE registers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "name" VARCHAR NOT NULL,
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('registers');

--


CREATE TABLE entities_files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id UUID NOT NULL,
    "filename" VARCHAR NOT NULL,
    "url" VARCHAR NOT NULL,
    content VARCHAR DEFAULT '',
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('entities_files');

--


CREATE TABLE entities_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    entity_id UUID NOT NULL,
    action_id SMALLINT NOT NULL,
    file_id UUID NOT NULL REFERENCES entities_files(id),
    user_id UUID NOT NULL REFERENCES users(id),
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('entities_history');

--


CREATE TABLE entities_notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    note VARCHAR DEFAULT '' NOT NULL,
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

SELECT diesel_manage_updated_at('entities_notes');

--
