CREATE TABLE items (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT,
    quantity INTEGER DEFAULT 1,
    created_date DATETIME NOT NULL,
    modified_date DATETIME NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE containers (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT,
    created_date DATETIME NOT NULL
);

CREATE TABLE item_locations (
    item_uuid TEXT ,
    container_uuid TEXT,
    PRIMARY KEY (item_uuid, container_uuid)
    FOREIGN KEY (item_uuid)
        REFERENCES items (item_uuid)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION,
    FOREIGN KEY (container_uuid)
        REFERENCES containers (container_uuid)
            ON DELETE CASCADE 
            ON UPDATE NO ACTION
);

/*
CREATE TABLE audit (
    id INTEGER PRIMARY KEY,
    table TEXT NOT NULL,
    field TEXT NOT NULL,
    record_id BLOB NOT NULL,
    old_value BLOB NOT NULL,
    new_value BLOB NOT NULL,
    who TEXT NOT NULL,
    date DATETIME NOT NULL
);

CREATE TABLE audit_creation (
    id INTEGER PRIMARY KEY,
    table TEXT NOT NULL,
    record_id BLOB NOT NULL,
    who TEXT NOT NULL,
    date DATETIME NOT NULL
);*/