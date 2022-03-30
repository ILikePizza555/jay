import { randomUUID } from "crypto";
import { DatabaseConnection } from "./db";
import { AmbiguiousContainerNameError, ContainerNameNotFoundError, ContainerUUIDNotFoundError } from "./error";
import { isUuidv4 } from "./useful";

export function createContainerAction(db: DatabaseConnection, name: string, location?: string, description?: string, type: string = "DEFAULT") {
    const result = db.insertContainerStatement.run({
        uuid: randomUUID(),
        name,
        description: description ?? null,
        type,
        created_date: (new Date()).getTime()
    });
}

export function queryContainerAction(db: DatabaseConnection, name_or_id: string) {
    if (isUuidv4(name_or_id)) {
        const container = db.getContainerByUuidStatement.get(name_or_id);

        if (!container) {
            throw new ContainerUUIDNotFoundError(name_or_id);
        }

        return container;
    }

    const container_set = db.getContainerByNameStatement.all(name_or_id);

    if (!container_set) {
        throw new ContainerNameNotFoundError(name_or_id);
    } else if (container_set.length > 1) {
        throw new AmbiguiousContainerNameError(name_or_id, container_set);
    }

    return container_set[0];
}