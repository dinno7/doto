-- Add up migration script here
CREATE TABLE todos (
    id integer PRIMARY KEY AUTOINCREMENT,
    title text NOT NULL,
    description text,
    priority text NOT NULL,
    status text NOT NULL,
    due_at text,
    completed_at text,
    updated_at text NOT NULL,
    created_at text NOT NULL
);

