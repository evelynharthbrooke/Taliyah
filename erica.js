const config = require('./config.json');

const EricaClient = require("./src/structs/EricaClient");

const client = new EricaClient(config);

client.start();
