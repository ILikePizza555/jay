CREATE TABLE items_history (
    id INTEGER PRIMARY KEY ASC,
    'from' DATETIME DEFAULT (datetime()),
    'to' DATETIME DEFAULT NULL,
    uuid BLOB NOT NULL,
    name TEXT,
    description TEXT,
    type TEXT,
    quantity INTEGER,
    status TEXT,
    deleted BOOLEAN DEFAULT 0
);

CREATE TRIGGER IF NOT EXISTS update_items_date_on_insert AFTER INSERT ON items_history
BEGIN
    UPDATE items_history
    SET 'to' = NEW.'from'
    WHERE id = (SELECT max(id) FROM items_history WHERE id != NEW.id AND uuid = NEW.uuid);
END;

CREATE TABLE containers_history (
    id INTEGER PRIMARY KEY ASC,
    'from' DATETIME DEFAULT (datetime()),
    'to' DATETIME DEFAULT NULL
    uuid BLOB NOT NULL,
    name TEXT,
    type TEXT,
    deleted BOOLEAN DEFAULT 0,
);