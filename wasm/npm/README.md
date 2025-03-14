# EntropyX WASM SDK

An integration wrapper around [`entropyx-wasm`](https://www.npmjs.com/package/entropyx-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the EntropyX WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/entropyxnet/rusty-entropyx/releases](https://github.com/entropyxnet/rusty-entropyx/releases)

## Usage

EntropyX NPM module exports include all WASM32 bindings.
```javascript
const entropyx = require('entropyx');
console.log(entropyx.version());
```

## Documentation

Documentation is available at [https://entropyx.aspectron.org/docs/](https://entropyx.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/entropyxnet/rusty-entropyx/tree/master/wasm](https://github.com/entropyxnet/rusty-entropyx/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/entropyxnet/rusty-entropyx/releases](https://github.com/entropyxnet/rusty-entropyx/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/entropyx-wasm.html](https://aspectron.org/en/projects/entropyx-wasm.html)

