import Sqlite3 from "better-sqlite3";

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

    public static openConnection(filename: string) {
        return new DatabaseConnection(new Sqlite3(filename));
    }

    constructor(db: Sqlite3.Database) {
        this._db = db;

        this.insertContainerStatement = this._db.prepare(
            "INSERT INTO containers (uuid, name, description, type, created_date) VALUES (@uuid, @name, @description, @type, @created_date);");
    }

    public destroy() {
        this._db.close();
    }
}