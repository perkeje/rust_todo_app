-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE tasks(
    id VARCHAR(36) DEFAULT uuid_generate_v4(),
    content TEXT NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    done BOOLEAN DEFAULT false NOT NULL,

    CONSTRAINT pk_todos PRIMARY KEY(id),
    CONSTRAINT fk_todos FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);