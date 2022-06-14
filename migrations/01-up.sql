-- Entities

CREATE TABLE IF NOT EXISTS users (
    uuid UUID DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    password BYTEA NOT NULL,
    last_login TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_users PRIMARY KEY (uuid),
    CONSTRAINT uk_users_email UNIQUE (email)
);

CREATE TABLE IF NOT EXISTS investigations (
    uuid UUID DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_investigations PRIMARY KEY (uuid),
    CONSTRAINT uk_investigations_name UNIQUE (name)
);

-- Relationships

CREATE TABLE users_investigations (
    user_uuid UUID NOT NULL,
    investigation_uuid UUID NOT NULL,

    CONSTRAINT pk_users_investigations PRIMARY KEY (user_uuid, investigation_uuid),
    CONSTRAINT fk_users_investigations_users FOREIGN KEY (user_uuid) REFERENCES users (uuid) ON DELETE CASCADE,
    CONSTRAINT fk_users_investigations_investigations FOREIGN KEY (investigation_uuid) REFERENCES investigations (uuid) ON DELETE CASCADE
);