import { Command } from "commander";
import { createContainerAction, queryContainerAction } from "./actions";
import { DatabaseConnection } from "./db";
import { DisplayError } from "./error";
import { handleDisplayError } from "./useful";

const db = DatabaseConnection.openConnection("jay.db")
const cli = new Command();

const createCommand = cli.command("create")
createCommand.command("container")
    .description("Creates a new container.")
    .option("-d, --description <description>", "A description of the object.")
    .option("-t, --type <type>", "The type or category of the object being created.")
    .argument("<name>", "The name of the container.")
    .argument("[location]", "The location of the container. Must either be a container name or id.")
    .action((name, location, options) => {
        createContainerAction(db, name, location, options.description, options.type);
        console.error("Created new container: " + name);
    });
createCommand.command("item")
    .description("Creates a new item.")
    .option("-d, --description <description>", "A description of the object.")
    .option("-t, --type <type>", "The type or category of the object being created.")
    .argument("<name>", "The name of the item.")
    .argument("<location>", "The location of the item. Must either be a container name or id.")
    .argument("[quantity]", "Quantity of the item. Must be greater than zero.", 1);
    
const listCommand = cli.command("list");
listCommand.command("all")
    .description("Lists all the objects in the catalogue.")
    .action(() => {
        console.table(db.selectAllStatement.all());
    });
listCommand.command("container")
    .description("Lists all items in the specified container.")
    .argument("<name-or-id>", "The name or id of a container.")
    .action(handleDisplayError((name_or_id: string) => {
        queryContainerAction(db, name_or_id)
    }));

cli.command("delete")
    .description("Deletes an object.")
    .argument("<name-or-id>", "The name or id of an object.");

cli.parse();
