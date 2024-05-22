-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id         INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    username   VARCHAR(32) NOT NULL UNIQUE,
    password   VARCHAR(128) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  INT NOT NULL,
    token    VARCHAR(32) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
