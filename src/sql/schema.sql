CREATE TABLE items_history (
    id INTEGER PRIMARY KEY ASC,
    'from' DATETIME DEFAULT (datetime()),
    'to' DATETIME DEFAULT NULL,
    who TEXT NOT NULL,
    uuid BLOB NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT,
    quantity INTEGER,
    status TEXT,
    deleted BOOLEAN DEFAULT 0
);

CREATE TRIGGER update_items_date_on_insert AFTER INSERT ON items_history
BEGIN
    UPDATE items_history
    SET 'to' = NEW.'from'
    WHERE id = (SELECT max(id) FROM items_history WHERE id != NEW.id AND uuid = NEW.uuid);
END;

CREATE VIEW current_items AS
SELECT
    max(id) as item_history_id,
    items_history.'from' as last_modified,
    uuid,
    name,
    description,
    type,
    quantity,
    status,
    deleted
FROM
    items_history
GROUP BY
    uuid;

CREATE TABLE containers_history (
    id INTEGER PRIMARY KEY ASC,
    'from' DATETIME DEFAULT (datetime()),
    'to' DATETIME DEFAULT NULL,
    uuid BLOB NOT NULL,
    name TEXT,
    type TEXT,
    deleted BOOLEAN DEFAULT 0
);