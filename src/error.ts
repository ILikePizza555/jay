import { table } from "table";

/**
 * An error type has a user-friendly message to display to the user.
 */
export abstract class DisplayError extends Error {
    abstract get userFriendlyError(): string
}

export class ContainerUUIDNotFoundError extends DisplayError {
    constructor(providedUuid: string) {
        super(`Container uuid "${providedUuid} does not exist."`)
    }

    get userFriendlyError(): string {
        return this.message;
    }
}

export class ContainerNameNotFoundError extends DisplayError {
    constructor(providedName: string) {
        super(`Container name "${providedName} does not exist."`)
    }

    get userFriendlyError(): string {
        return this.message;
    }
}

export class AmbiguiousContainerNameError extends DisplayError {
    public readonly providedName: string;
    public readonly container_set: any[];
    
    constructor(providedName: string, container_set: any[]) {
        super(`Found ${container_set.length} containers with the name "${providedName}".`)

        this.providedName = providedName;
        this.container_set = container_set;
    }

    get userFriendlyError(): string {
        const containerTable = table(
            [["uuid", "description", "type"]].concat(
                this.container_set.map(c => [c.uuid, c.description, c.type])
            ),
            {singleLine: true}
        );

        return this.message.concat("\n", containerTable);
    }
}