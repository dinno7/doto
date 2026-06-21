-- Add up migration script here
CREATE TABLE tags (
    id integer PRIMARY KEY AUTOINCREMENT,
    name text NOT NULL UNIQUE
);

CREATE TABLE todo_tags (
    tag_id integer NOT NULL,
    todo_id integer NOT NULL,
    PRIMARY KEY (tag_id, todo_id),
    FOREIGN KEY (todo_id) REFERENCES todos (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

