/*!
# `vecno WASM32 bindings`

<br>

Vecno WASM32 bindings offer direct integration of Rust code and Vecno
codebase within JavaScript environments such as Node.js and Web Browsers.

Please note that while WASM directly binds JavaScript and Rust resources, their names on JavaScript side
are different from their name in Rust as they conform to the 'camelCase' convention in JavaScript and
to the 'snake_case' convention in Rust.

## Interfaces

The APIs are currently separated into the following groups (this will be expanded in the future):

- **Transaction API** — Bindings for primitives related to transactions.
- **RPC API** — [RPC interface bindings](rpc) for the Vecno node using WebSocket (wRPC) connections.
- **Wallet API** — API for async core wallet processing tasks.

## NPM Modules

For JavaScript / TypeScript environments, there are two
available NPM modules:

- <https://www.npmjs.com/package/vecno>
- <https://www.npmjs.com/package/vecno-wasm>

The `vecno-wasm` module is a pure WASM32 module that includes
the entire wallet framework, but does not support RPC due to an absence
of a native WebSocket in NodeJs environment, while
the `vecno` module includes `websocket` package dependency simulating
the W3C WebSocket and due to this supports RPC.


## Using RPC

**NODEJS:** If you are building from source, to use WASM RPC client
in the NodeJS environment, you need to introduce a global W3C WebSocket
object before loading the WASM32 library (to simulate the browser behavior).
You can the [WebSocket](https://www.npmjs.com/package/websocket)
module that offers W3C WebSocket compatibility and is compatible
with Vecno RPC implementation.

You can use the following shims:

```js
// WebSocket
globalThis.WebSocket = require('websocket').w3cwebsocket;
```

## Loading in a Web App

```html
<html>
    <head>
        <script type="module">
            import * as vecno_wasm from './vecno/vecno-wasm.js';
            (async () => {
                const vecno = await vecno_wasm.default('./vecno/vecno-wasm_bg.wasm');
                // ...
            })();
        </script>
    </head>
    <body></body>
</html>
```

## Loading in a Node.js App

```javascript
// W3C WebSocket module shim
// this is provided by NPM `vecno` module and is only needed
// if you are building WASM libraries for NodeJS from source
// globalThis.WebSocket = require('websocket').w3cwebsocket;

let {RpcClient,Encoding,initConsolePanicHook} = require('./vecno-rpc');

// enabling console panic hooks allows WASM to print panic details to console
// initConsolePanicHook();
// enabling browser panic hooks will create a full-page DIV with panic details
// this is useful for mobile devices where console is not available
// initBrowserPanicHook();

// if port is not specified, it will use the default port for the specified network
const rpc = new RpcClient("127.0.0.1", Encoding.Borsh, "testnet-10");
const rpc = new RpcClient({
    url : "127.0.0.1",
    encoding : Encoding.Borsh,
    networkId : "testnet-10"
});


(async () => {
    try {
        await rpc.connect();
        let info = await rpc.getInfo();
        console.log(info);
    } finally {
        await rpc.disconnect();
    }
})();
```

*/

#![allow(unused_imports)]

#[cfg(all(
    any(feature = "wasm32-sdk", feature = "wasm32-rpc", feature = "wasm32-core", feature = "wasm32-keygen"),
    not(target_arch = "wasm32")
))]
compile_error!("`vecno-wasm` crate for WASM32 target must be built with `--features wasm32-sdk|wasm32-rpc|wasm32-core|wasm32-keygen`");

mod version;
pub use version::*;

cfg_if::cfg_if! {

    if #[cfg(feature = "wasm32-sdk")] {

        pub use vecno_addresses::{Address, Version as AddressVersion};
        pub use vecno_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use vecno_pow::wasm::*;

        pub mod rpc {
            //! Vecno RPC interface
            //!

            pub mod messages {
                //! Vecno RPC messages
                pub use vecno_rpc_core::model::message::*;
            }
            pub use vecno_rpc_core::api::rpc::RpcApi;
            pub use vecno_rpc_core::wasm::message::*;

            pub use vecno_wrpc_wasm::client::*;
            pub use vecno_wrpc_wasm::resolver::*;
            pub use vecno_wrpc_wasm::notify::*;
        }

        pub use vecno_consensus_wasm::*;
        pub use vecno_wallet_keys::prelude::*;
        pub use vecno_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-core")] {

        pub use vecno_addresses::{Address, Version as AddressVersion};
        pub use vecno_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use vecno_pow::wasm::*;

        pub mod rpc {
            //! Vecno RPC interface
            //!

            pub mod messages {
                //! Vecno RPC messages
                pub use vecno_rpc_core::model::message::*;
            }
            pub use vecno_rpc_core::api::rpc::RpcApi;
            pub use vecno_rpc_core::wasm::message::*;

            pub use vecno_wrpc_wasm::client::*;
            pub use vecno_wrpc_wasm::resolver::*;
            pub use vecno_wrpc_wasm::notify::*;
        }

        pub use vecno_consensus_wasm::*;
        pub use vecno_wallet_keys::prelude::*;
        pub use vecno_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-rpc")] {

        pub use vecno_rpc_core::api::rpc::RpcApi;
        pub use vecno_rpc_core::wasm::message::*;
        pub use vecno_rpc_core::wasm::message::IPingRequest;
        pub use vecno_wrpc_wasm::client::*;
        pub use vecno_wrpc_wasm::resolver::*;
        pub use vecno_wrpc_wasm::notify::*;
        pub use vecno_wasm_core::types::*;

    } else if #[cfg(feature = "wasm32-keygen")] {

        pub use vecno_addresses::{Address, Version as AddressVersion};
        pub use vecno_wallet_keys::prelude::*;
        pub use vecno_wasm_core::types::*;

    }
}
