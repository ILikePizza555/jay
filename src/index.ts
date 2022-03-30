import { Command } from "commander";
import { DatabaseConnection } from "./db";
import { UUID } from "./uuid";

DatabaseConnection.openConnection("jay.db").then(db => {
    const cli = new Command();

    const createCommand = cli.command("create")
    createCommand.command("container")
        .description("Creates a new container.")
        .option("-d, --description <description>", "A description of the object.")
        .option("-t, --type <type>", "The type or category of the object being created.")
        .argument("<name>", "The name of the container.")
        .argument("[location]", "The location of the container. Must either be a container name or id.")
        .action((name, location, options) => {
            db.insertContainer({
                uuid: new UUID(),
                name: name,
                description: options.description ?? null,
                type: options.type ?? "DEFAULT",
                created_date: new Date()
            });
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
        .description("Lists all the objects in the catalogue.");
    listCommand.command("container")
        .description("Lists all items in the specified container.")
        .argument("<name-or-id>", "The name or id of a container.");

    cli.command("delete")
        .description("Deletes an object.")
        .argument("<name-or-id>", "The name or id of an object.");

    cli.parse();
});