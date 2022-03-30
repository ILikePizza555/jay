import { randomUUID } from "crypto";
import { DatabaseConnection } from "./db";

export function createContainerAction(db: DatabaseConnection, name: string, location?: string, description?: string, type: string = "DEFAULT") {
    const result = db.insertContainerStatement.run({
        uuid: randomUUID(),
        name,
        description: description ?? null,
        type,
        created_date: (new Date()).getTime()
    });
}