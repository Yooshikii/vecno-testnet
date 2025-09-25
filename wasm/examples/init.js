const fs = require('fs');
const path = require('path');
const { Mnemonic, XPrv, PublicKeyGenerator } = require('../nodejs/vecno');
const { parseArgs } = require('node:util');

let args = process.argv.slice(2);
const {
    values,
    positionals,
} = parseArgs({
    args,
    options: {
        help: { type: 'boolean' },
        reset: { type: 'boolean' },
        network: { type: 'string' },
    },
    tokens: true,
    allowPositionals: true,
});

if (values.help) {
    console.log(`Usage: node init [--reset] [--network=(mainnet|testnet-<number>)]`);
    process.exit(0);
}

const network = values.network ?? positionals.find((positional) => positional.match(/^(testnet|mainnet|simnet|devnet)-\d*$/)) ?? null;
const configFileName = path.join(__dirname, "data", "config.json");

// Ensure data directory exists
const dataDir = path.join(__dirname, "data");
if (!fs.existsSync(dataDir)) {
    fs.mkdirSync(dataDir, { recursive: true });
}

if (!fs.existsSync(configFileName) || values.reset) {
    createConfigFile();
    process.exit(0);
}

if (network) {
    try {
        let config = JSON.parse(fs.readFileSync(configFileName, "utf8"));
        config.networkId = network;
        let wallet = basicWallet(network, new Mnemonic(config.mnemonic));
        config.privateKey = wallet.privateKey;
        config.xprv = wallet.xprv;
        config.receive = wallet.receive;
        config.change = wallet.change;
        fs.writeFileSync(configFileName, JSON.stringify(config, null, 4));
        fs.chmodSync(configFileName, 0o600);
        console.log(`Updated networkId to '${network}' and wallet data in config.json`);
    } catch (error) {
        console.error("Error updating config file:", error.message);
        process.exit(1);
    }
}

if (fs.existsSync(configFileName)) {
    try {
        let config = JSON.parse(fs.readFileSync(configFileName, "utf8"));
        console.log("Loading mnemonic:", config.mnemonic);
        let mnemonic = new Mnemonic(config.mnemonic);
        let wallet = basicWallet(config.networkId, mnemonic);

        console.log("");
        console.log("networkId:", config.networkId);
        console.log("mnemonic:", wallet.mnemonic.phrase);
        console.log("xprv:", wallet.xprv);
        console.log("receive:", wallet.receive);
        console.log("change:", wallet.change);
        console.log("privatekey:", wallet.privateKey);
        console.log("");
        console.log("WARNING: privatekey and mnemonic is sensitive. Secure the config.json file and avoid sharing it.");
        console.log("Use 'init --reset' to reset the config file");
        console.log("");
    } catch (error) {
        console.error("Error reading config file:", error.message);
        process.exit(1);
    }
}

function createConfigFile() {
    if (!network) {
        console.log("... '--network=' argument is not specified ...defaulting to 'mainnet'");
    }
    let networkId = network ?? "mainnet";

    let wallet = basicWallet(networkId, Mnemonic.random());

    let config = {
        networkId,
        mnemonic: wallet.mnemonic.phrase,
        privateKey: wallet.privateKey,
        xprv: wallet.xprv,
        receive: wallet.receive,
        change: wallet.change,
    };
    try {
        fs.writeFileSync(configFileName, JSON.stringify(config, null, 4));
        fs.chmodSync(configFileName, 0o600);
        console.log("Created config data in './data/config.json'");
        console.log("");
        console.log("networkId:", networkId);
        console.log("mnemonic:", wallet.mnemonic.phrase);
        console.log("xprv:", wallet.xprv);
        console.log("receive:", wallet.receive);
        console.log("change:", wallet.change);
        console.log("privatekey:", wallet.privateKey);
        console.log("");
        console.log("WARNING: privatekey and mnemonic is sensitive. Secure the config.json file and avoid sharing it.");
    } catch (error) {
        console.error("Error creating config file:", error.message);
        process.exit(1);
    }
}

function basicWallet(networkId, mnemonic) {
    console.log("mnemonic:", mnemonic.phrase);
    let xprv = new XPrv(mnemonic.toSeed());
    let account_0_root = xprv
        .deriveChild(44, true)
        .deriveChild(111111, true)
        .deriveChild(0, true)
        .toXPub();
    let account_0 = {
        receive_xpub: account_0_root.deriveChild(0, false),
        change_xpub: account_0_root.deriveChild(1, false),
    };
    let receive = account_0.receive_xpub.deriveChild(0, false).toPublicKey().toAddress(networkId).toString();
    let change = account_0.change_xpub.deriveChild(0, false).toPublicKey().toAddress(networkId).toString();

    let privateKey = xprv
        .deriveChild(44, true)
        .deriveChild(111111, true)
        .deriveChild(0, true)
        .deriveChild(0, false)
        .deriveChild(0, false)
        .toPrivateKey()
        .toString();

    PublicKeyGenerator.fromMasterXPrv(
        xprv.toString(),
        false,
        0n,
        0
    );

    return {
        mnemonic,
        xprv: xprv.toString(),
        receive,
        change,
        privateKey,
    };
}