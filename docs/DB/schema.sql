-- SQL schema for Yukthi File Storage (YFS)

-- Users
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    domain VARCHAR(255) NOT NULL,
    -- TODO: CalDav authentication integration
    -- caldav_app_password_hash TEXT,

    organization_id UUID NOT NULL,
    organization_name VARCHAR(250) NOT NULL,

    private_info JSONB NOT NULL,    -- Available only to the user themselves
    public_info JSONB NOT NULL,     -- Available to all within the organization

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);


-- User Session management
CREATE TABLE sessions (
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    refresh_token UUID PRIMARY KEY,

    sso_token VARCHAR(50) NOT NULL,
    fcm_token TEXT NULL,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP + INTERVAL '3 days')
);


-- Enum of task status
CREATE TYPE task_status AS ENUM (
    'PENDING',      -- Not yet started
    'IN_PROGRESS',  -- Started / Work in progress
    'COMPLETED',    -- Finished successfully
    'CANCELLED',    -- Cancelled / Rejected
    'ON_HOLD'       -- Temporarily paused / Stalled / On-hold
);


-- Tasks
CREATE TABLE tasks (
    task_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    parent_task_id BIGINT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,

    title VARCHAR(255) NOT NULL,
    description TEXT,
    details JSONB NOT NULL, -- UI colours, etc.

    task_status task_status NOT NULL DEFAULT 'PENDING',

    start_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_at TIMESTAMPTZ NOT NULL,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- Ensure that a user cannot have multiple tasks with the same title under the same parent task
    UNIQUE (owner_id, parent_task_id, title)
);


-- Alerts for tasks
CREATE TABLE task_alerts (
    task_id BIGINT NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,

    -- The user who should receive the alert
    alert_recipient_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    -- The timestamp when the alert should trigger
    alert_at TIMESTAMPTZ NOT NULL,

    notify_via_email BOOLEAN NOT NULL DEFAULT FALSE,    -- Whether to notify via email
    notify_via_push BOOLEAN NOT NULL DEFAULT FALSE,     -- Whether to notify via push notification

    -- Used by the background job to track if the notification has been sent
    notification_sent BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- Composite primary key ensures uniqueness of alerts for the same task and recipient at the same time
    PRIMARY KEY (task_id, alert_recipient_id, alert_at)
);


-- Recurring Tasks (This is like a template for tasks that repeat over time)
-- UI will use this to generate a task if the user want to edit certain occurrences of the recurring task
CREATE TABLE recurring_tasks (
    recurring_task_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    title VARCHAR(255) NOT NULL,
    description TEXT,
    details JSONB NOT NULL, -- UI colours, etc.

    -- RFC 5545 RRULE
    rrule TEXT NOT NULL, -- Stores the recurrence rule in RFC 5545 format

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- Ensure that a user cannot have multiple recurring tasks with the same title
    UNIQUE (owner_id, title)
);


-- Task Assignees (Mapping of tasks to users)
CREATE TABLE task_assignees (
    task_id BIGINT NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,

    -- The user who is assigned to the task
    assignee_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    -- The user who assigned the task
    assigned_by UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    -- Composite primary key ensures uniqueness of task assignments for each user
    PRIMARY KEY (task_id, assignee_id)
);


-- Comments on tasks (PK is not comment_id because we always include task_id with it)
CREATE TABLE task_comments (
    comment_id BIGINT GENERATED ALWAYS AS IDENTITY,
    task_id BIGINT NOT NULL REFERENCES tasks(task_id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    content TEXT NOT NULL,
    reactions JSONB NULL, -- Stores reactions to the comment, e.g., likes, emojis

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (task_id, comment_id)
);


-- Task Views
CREATE TABLE task_views (
    view_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    view_name VARCHAR(255) NOT NULL,
    description TEXT NULL,
    ui_info JSONB NULL, -- Stores UI-related information, e.g., layout, colors

    -- Filter by info
    status_filter task_status[] NULL,               -- Filter tasks by their status (Any-one)
    show_recurring BOOLEAN NOT NULL DEFAULT FALSE,  -- Show recurring tasks or not
    show_comments BOOLEAN NOT NULL DEFAULT FALSE,   -- Show comments for any tasks
    show_subtasks BOOLEAN NOT NULL DEFAULT FALSE,   -- Just list top-level tasks
    show_assigned BOOLEAN NOT NULL DEFAULT FALSE,   -- Show tasks assigned to the view owner_id

    UNIQUE (owner_id, view_name)
);


-- Shared Views
CREATE TABLE shared_views (
    view_id BIGINT NOT NULL REFERENCES task_views(view_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    share_notes TEXT NULL, -- Notes or comments about the shared view
    ui_info JSONB NULL, -- Stores UI-related information for the shared view, e.g., layout, colors

    -- If the user has certain permissions for the shared view
    can_create BOOLEAN DEFAULT FALSE NOT NULL,
    can_edit BOOLEAN DEFAULT FALSE NOT NULL,
    can_delete BOOLEAN DEFAULT FALSE NOT NULL,

    shared_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY (view_id, user_id)
);


-- Indexes for performance optimization
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_sso_token ON sessions(sso_token);
