import { Command } from "commander";

const cli = new Command();

const createCommand = cli.command("create")
    .option("-d, --description <description>", "A description of the object.")
    .option("-t, --type <type>", "The type or category of the object being created.");

createCommand.command("container")
    .description("Creates a new container.")
    .argument("<name>", "The name of the container.")
    .argument("[location]", "The location of the container. Must either be a container name or id.");
createCommand.command("item")
    .description("Creates a new item.")
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