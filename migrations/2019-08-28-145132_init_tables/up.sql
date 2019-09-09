-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


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


CREATE TABLE organizations ( -- organization
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    org_name VARCHAR DEFAULT '' NOT NULL,
    profile_picture BYTEA DEFAULT '' NOT NULL,
    coordinates_id UUID REFERENCES coordinates(id),
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('organizations');


--


CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    org_id UUID REFERENCES organizations(id) NOT NULL,
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

CREATE TABLE building_owners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    full_name VARCHAR DEFAULT '' NOT NULL,
    is_manager BOOL DEFAULT FALSE NOT NULL,
    org_id UUID REFERENCES organizations(id),
    linked_user_id UUID REFERENCES users(id),
    coordinates_id UUID REFERENCES coordinates(id),
    deleted BOOL DEFAULT 0::BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('building_owners');


--

CREATE TABLE buildings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    owner_id UUID NOT NULL REFERENCES building_owners(id),
    org_id UUID NOT NULL REFERENCES organizations(id),
    respondant_id UUID NOT NULL REFERENCES users(id),
    "name" VARCHAR DEFAULT '' NOT NULL,
    "address" VARCHAR DEFAULT '' NOT NULL,
    deleted BOOL DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('buildings');

--

CREATE TABLE registers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    "name" VARCHAR DEFAULT '' NOT NULL,
    building_id UUID NOT NULL REFERENCES buildings(id),
    deleted BOOL DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('registers');

--

CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "filename" VARCHAR NOT NULL,
    "url" VARCHAR NOT NULL,
    content VARCHAR DEFAULT '' NOT NULL,
    deleted BOOL DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('files');

--

CREATE TABLE entities_files (
    file_id UUID NOT NULL REFERENCES files(id),
    entity_id UUID NOT NULL,
    PRIMARY KEY (file_id, entity_id),
    deleted BOOL DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('entities_files');

--

CREATE TABLE entities_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    entity_id UUID NOT NULL,
    action_id SMALLINT NOT NULL,
    file_id UUID NOT NULL REFERENCES files(id),
    user_id UUID NOT NULL REFERENCES users(id),
    deleted BOOL DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('entities_history');

--


CREATE TABLE entities_notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    note VARCHAR DEFAULT '' NOT NULL,
    deleted BOOL DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT TO_TIMESTAMP(0) NOT NULL
);

SELECT diesel_manage_updated_at('entities_notes');

--
