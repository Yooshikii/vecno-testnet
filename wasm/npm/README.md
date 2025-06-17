# Vecno WASM SDK

An integration wrapper around [`vecno-wasm`](https://www.npmjs.com/package/vecno-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the Vecno WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/vecno-foundation/vecno-testnet/releases](https://github.com/vecno-foundation/vecno-testnet/releases)

## Usage

Vecno NPM module exports include all WASM32 bindings.
```javascript
const vecno = require('vecno');
console.log(vecno.version());
```

## Documentation

Documentation is available at [https://vecno.aspectron.org/docs/](https://vecno.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/vecno-foundation/vecno-testnet/tree/master/wasm](https://github.com/vecno-foundation/vecno-testnet/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/vecno-foundation/vecno-testnet/releases](https://github.com/vecno-foundation/vecno-testnet/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/vecno-wasm.html](https://aspectron.org/en/projects/vecno-wasm.html)

