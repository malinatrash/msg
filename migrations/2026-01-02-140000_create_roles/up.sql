CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO roles (id, name, description) VALUES
    (0, 'admin', 'Administrator with full access'),
    (1, 'user', 'Default user role');
