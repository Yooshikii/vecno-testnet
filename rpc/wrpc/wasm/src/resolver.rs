//! [`Resolver`](NativeResolver) bindings for obtaining public Vecno wRPC URL endpoints.

#![allow(non_snake_case)]

use crate::client::{RpcClient, RpcConfig};
use crate::imports::*;
use js_sys::Array;
use serde::ser;
pub use vecno_rpc_macros::declare_typescript_wasm_interface as declare;
use vecno_wrpc_client::node::NodeDescriptor;
use vecno_wrpc_client::Resolver as NativeResolver;
use workflow_wasm::extensions::ObjectExtension;

declare! {
    IResolverConfig,
    "IResolverConfig | string[]",
    r#"
    /**
     * RPC Resolver configuration options
     * 
     * @category Node RPC
     */
    export interface IResolverConfig {
        /**
         * Optional URLs for one or multiple resolvers.
         */
        urls?: string[];
        /**
         * Use strict TLS for RPC connections.
         * If not set or `false` (default), the resolver will
         * provide the best available connection regardless of
         * whether this connection supports TLS or not.
         * If set to `true`, the resolver will only provide
         * TLS-enabled connections.
         * 
         * This setting is ignored in the browser environment
         * when the browser navigator location is `https`.
         * In which case the resolver will always use TLS-enabled
         * connections.
         */
        tls?: boolean;
    }
    "#,
}

declare! {
    IResolverConnect,
    "IResolverConnect | NetworkId | string",
    r#"
    /**
     * RPC Resolver connection options
     * 
     * @category Node RPC
     */
    export interface IResolverConnect {
        /**
         * RPC encoding: `borsh` (default) or `json`
         */
        encoding?: Encoding | string;
        /**
         * Network identifier: `mainnet` or `testnet-11` etc.
         */
        networkId?: NetworkId | string;
    }
    "#,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverConnect {
    pub encoding: Option<Encoding>,
    pub network_id: NetworkId,
}

impl TryFrom<IResolverConnect> for ResolverConnect {
    type Error = Error;
    fn try_from(config: IResolverConnect) -> Result<Self> {
        if let Ok(network_id) = NetworkId::try_owned_from(&config) {
            Ok(Self { encoding: None, network_id })
        } else {
            Ok(serde_wasm_bindgen::from_value(config.into())?)
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Array, typescript_type = "string[]")]
    pub type ResolverArrayT;
}

///
/// Resolver is a client for obtaining public Vecno wRPC URL.
///
/// Resolver queries a list of public Vecno Resolver URLs using HTTP to fetch
/// wRPC endpoints for the given encoding, network identifier and other
/// parameters. It then provides this information to the {@link RpcClient}.
///
/// Each time {@link RpcClient} disconnects, it will query the resolver
/// to fetch a new wRPC URL.
///
/// ```javascript
/// // using integrated public URLs
/// let rpc = RpcClient({
///     resolver: new Resolver(),
///     networkId : "mainnet"
/// });
///
/// // specifying custom resolver URLs
/// let rpc = RpcClient({
///     resolver: new Resolver({urls: ["<resolver-url>",...]}),
///     networkId : "mainnet"
/// });
/// ```
///
/// @see {@link IResolverConfig}, {@link IResolverConnect}, {@link RpcClient}
/// @category Node RPC
///
#[derive(Debug, Clone, CastFromJs)]
#[wasm_bindgen(inspectable)]
pub struct Resolver {
    resolver: NativeResolver,
}

impl Resolver {
    pub fn new(resolver: NativeResolver) -> Self {
        Self { resolver }
    }
}

#[wasm_bindgen]
impl Resolver {
    /// Creates a new Resolver client with the given
    /// configuration supplied as {@link IResolverConfig}
    /// interface. If not supplied, the default configuration
    /// containing a list of community-operated resolvers
    /// will be used.
    #[wasm_bindgen(constructor)]
    pub fn ctor(args: Option<IResolverConfig>) -> Result<Resolver> {
        if let Some(args) = args {
            Ok(Self { resolver: NativeResolver::try_from(args)? })
        } else {
            Ok(Self { resolver: NativeResolver::default() })
        }
    }
}

#[wasm_bindgen]
impl Resolver {
    /// List of public Vecno Resolver URLs.
    #[wasm_bindgen(getter)]
    pub fn urls(&self) -> Option<ResolverArrayT> {
        self.resolver.urls().map(|urls| Array::from_iter(urls.iter().map(|v| JsValue::from(v.as_str()))).unchecked_into())
    }

    /// Fetches a public Vecno wRPC endpoint for the given encoding and network identifier.
    /// @see {@link Encoding}, {@link NetworkId}, {@link NodeDescriptor}
    #[wasm_bindgen(js_name = getNode)]
    pub async fn get_node(&self, encoding: Encoding, network_id: NetworkIdT) -> Result<NodeDescriptor> {
        self.resolver.get_node(encoding, *network_id.try_into_cast()?).await
    }

    /// Fetches a public Vecno wRPC endpoint URL for the given encoding and network identifier.
    /// @see {@link Encoding}, {@link NetworkId}
    #[wasm_bindgen(js_name = getUrl)]
    pub async fn get_url(&self, encoding: Encoding, network_id: NetworkIdT) -> Result<String> {
        self.resolver.get_url(encoding, *network_id.try_into_cast()?).await
    }

    /// Connect to a public Vecno wRPC endpoint for the given encoding and network identifier
    /// supplied via {@link IResolverConnect} interface.
    /// @see {@link IResolverConnect}, {@link RpcClient}
    pub async fn connect(&self, options: IResolverConnect) -> Result<RpcClient> {
        let ResolverConnect { encoding, network_id } = options.try_into()?;
        let config = RpcConfig { resolver: Some(self.clone()), url: None, encoding, network_id: Some(network_id) };
        let client = RpcClient::new(Some(config))?;
        client.connect(None).await?;
        Ok(client)
    }
}

impl TryFrom<IResolverConfig> for NativeResolver {
    type Error = Error;
    fn try_from(config: IResolverConfig) -> Result<Self> {
        let tls = config.get_bool("tls").unwrap_or(false);
        let urls = config
            .get_vec("urls")
            .map(|urls| urls.into_iter().map(|v| v.as_string()).collect::<Option<Vec<_>>>())
            .or_else(|_| config.dyn_into::<Array>().map(|urls| urls.into_iter().map(|v| v.as_string()).collect::<Option<Vec<_>>>()))
            .map_err(|_| Error::custom("Invalid or missing resolver URL"))?;

        if let Some(urls) = urls {
            Ok(NativeResolver::new(Some(urls.into_iter().map(Arc::new).collect()), tls))
        } else {
            Ok(NativeResolver::new(None, tls))
        }
    }
}

impl TryCastFromJs for Resolver {
    type Error = Error;
    fn try_cast_from<'a, R>(value: &'a R) -> Result<Cast<'a, Self>>
    where
        R: AsRef<JsValue> + 'a,
    {
        Ok(Self::try_ref_from_js_value_as_cast(value)?)
    }
}

impl TryFrom<&JsValue> for Resolver {
    type Error = Error;
    fn try_from(js_value: &JsValue) -> Result<Self> {
        Ok(Resolver::try_ref_from_js_value(js_value)?.clone())
    }
}

impl TryFrom<JsValue> for Resolver {
    type Error = Error;
    fn try_from(js_value: JsValue) -> Result<Self> {
        Resolver::try_from(js_value.as_ref())
    }
}

impl From<Resolver> for NativeResolver {
    fn from(resolver: Resolver) -> Self {
        resolver.resolver
    }
}

impl From<NativeResolver> for Resolver {
    fn from(resolver: NativeResolver) -> Self {
        Self { resolver }
    }
}
