-- SQL schema for this template

-- Notes
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- Indexes for performance optimization
CREATE INDEX idx_notes_created_at ON notes (created_at);
