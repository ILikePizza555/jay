import Sqlite3 from "better-sqlite3";

export const ITEMS_TABLE_NAME = "items";
export const CONTAINERS_TABLE_NAME = "containers";

export interface ItemDTO {
    uuid: string;
    name: string;
    description: string|null;
    type: string|null;
    quanitity: number;
    created_date: Date;
    modified_date: Date;
    status: string;
}

export interface InsertContainerQueryParameters {
    uuid: string;
    name: string;
    description: string|null;
    type: string;
    created_date: number;
}

export class DatabaseConnection {
    private _db: Sqlite3.Database;

    public readonly insertContainerStatement: Sqlite3.Statement<InsertContainerQueryParameters>;
    public readonly selectAllStatement: Sqlite3.Statement;

    public static openConnection(filename: string) {
        return new DatabaseConnection(new Sqlite3(filename));
    }

    constructor(db: Sqlite3.Database) {
        this._db = db;

        this.insertContainerStatement = this._db.prepare(
            "INSERT INTO containers (uuid, name, description, type, created_date) VALUES (@uuid, @name, @description, @type, @created_date);");

        this.selectAllStatement = this._db.prepare(
            `SELECT 'item' as object_type, uuid, name, description, type FROM ${ITEMS_TABLE_NAME}
            UNION
            SELECT 'container' as object_type, uuid, name, description, type FROM ${CONTAINERS_TABLE_NAME};`
        );
    }

    public destroy() {
        this._db.close();
    }
}