import BSqlite3 from "better-sqlite3";
import SQL from 'sql-template-strings'
import { Database, open } from "sqlite";
import { UUID } from "./uuid";
import { Buffer } from "buffer";

export interface ItemDTO {
    uuid: UUID;
    name: string;
    description: string|null;
    type: string|null;
    quanitity: number;
    created_date: Date;
    modified_date: Date;
    status: string;
}

export interface ContainerDTO {
    uuid: UUID;
    name: string;
    description: string|null;
    type: string;
    created_date: Date;
}

export class DatabaseConnection {
    private _db: Database;

    public static async openConnection(filename: string) {
        const db = await open({
            filename: filename,
            driver: BSqlite3
        });
        return new DatabaseConnection(db);
    }

    constructor(db: Database) {
        this._db = db;
    }

    public async createContainerRaw(rows: ContainerDTO[]) {
        const values_sql = rows.map(r => SQL`(${r.uuid.asBuffer}, ${r.name}, ${r.type}, ${r.created_date.getTime()})`).join(", ")
        return await this._db.run(SQL`INSERT INTO items (uuid, name, type, created_date) VALUES ${values_sql};`);
    }
}