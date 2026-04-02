-- =========================
-- ENUM TYPES
-- =========================

CREATE TYPE category_type AS ENUM ('income', 'expenses');

CREATE TYPE counterparty_type AS ENUM ('vendor', 'contractor', 'employee', 'client');

CREATE TYPE user_role AS ENUM ('admin', 'viewer', 'analyst');

CREATE TYPE user_status AS ENUM ('active', 'inactive');

CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed', 'cancelled');


-- =========================
-- TABLE: categories
-- =========================

CREATE TABLE categories (
                            id SERIAL PRIMARY KEY,
                            name VARCHAR(255) NOT NULL,
                            type category_type NOT NULL,
                            description TEXT,
                            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- =========================
-- TABLE: counterparties
-- =========================

CREATE TABLE counterparties (
                                id SERIAL PRIMARY KEY,
                                name VARCHAR(255) NOT NULL,
                                type counterparty_type NOT NULL,
                                email TEXT,
                                phone VARCHAR(20),
                                address TEXT,
                                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- =========================
-- TABLE: users
-- =========================

CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       name VARCHAR(255) NOT NULL,
                       email TEXT UNIQUE NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       role user_role NOT NULL DEFAULT 'viewer',
                       status user_status NOT NULL DEFAULT 'active',
                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- =========================
-- TABLE: transactions
-- =========================

CREATE TABLE transactions (
                              id SERIAL PRIMARY KEY,
                              amount FLOAT NOT NULL,
                              transaction_date DATE NOT NULL,
                              status transaction_status NOT NULL,

                              category_id INT NOT NULL,
                              counterparty_id INT NOT NULL,
                              created_by INT NOT NULL,

                              created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                              updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    -- FOREIGN KEYS
                              CONSTRAINT fk_category
                                  FOREIGN KEY (category_id)
                                      REFERENCES categories(id)
                                      ON DELETE RESTRICT,

                              CONSTRAINT fk_counterparty
                                  FOREIGN KEY (counterparty_id)
                                      REFERENCES counterparties(id)
                                      ON DELETE RESTRICT,

                              CONSTRAINT fk_user
                                  FOREIGN KEY (created_by)
                                      REFERENCES users(id)
                                      ON DELETE CASCADE
);