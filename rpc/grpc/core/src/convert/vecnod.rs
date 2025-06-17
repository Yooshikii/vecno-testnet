use crate::protowire::{vecnod_request, VecnodRequest, VecnodResponse};

impl From<vecnod_request::Payload> for VecnodRequest {
    fn from(item: vecnod_request::Payload) -> Self {
        VecnodRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<VecnodRequest> for VecnodRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<VecnodResponse> for VecnodResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod vecnod_request_convert {
    use crate::protowire::*;
    use vecno_rpc_core::{RpcError, RpcResult};

    impl_into_vecnod_request!(Shutdown);
    impl_into_vecnod_request!(SubmitBlock);
    impl_into_vecnod_request!(GetBlockTemplate);
    impl_into_vecnod_request!(GetBlock);
    impl_into_vecnod_request!(GetInfo);

    impl_into_vecnod_request!(GetCurrentNetwork);
    impl_into_vecnod_request!(GetPeerAddresses);
    impl_into_vecnod_request!(GetSink);
    impl_into_vecnod_request!(GetMempoolEntry);
    impl_into_vecnod_request!(GetMempoolEntries);
    impl_into_vecnod_request!(GetConnectedPeerInfo);
    impl_into_vecnod_request!(AddPeer);
    impl_into_vecnod_request!(SubmitTransaction);
    impl_into_vecnod_request!(SubmitTransactionReplacement);
    impl_into_vecnod_request!(GetSubnetwork);
    impl_into_vecnod_request!(GetVirtualChainFromBlock);
    impl_into_vecnod_request!(GetBlocks);
    impl_into_vecnod_request!(GetBlockCount);
    impl_into_vecnod_request!(GetBlockDagInfo);
    impl_into_vecnod_request!(ResolveFinalityConflict);
    impl_into_vecnod_request!(GetHeaders);
    impl_into_vecnod_request!(GetUtxosByAddresses);
    impl_into_vecnod_request!(GetBalanceByAddress);
    impl_into_vecnod_request!(GetBalancesByAddresses);
    impl_into_vecnod_request!(GetSinkBlueScore);
    impl_into_vecnod_request!(Ban);
    impl_into_vecnod_request!(Unban);
    impl_into_vecnod_request!(EstimateNetworkHashesPerSecond);
    impl_into_vecnod_request!(GetMempoolEntriesByAddresses);
    impl_into_vecnod_request!(GetCoinSupply);
    impl_into_vecnod_request!(Ping);
    impl_into_vecnod_request!(GetMetrics);
    impl_into_vecnod_request!(GetConnections);
    impl_into_vecnod_request!(GetSystemInfo);
    impl_into_vecnod_request!(GetServerInfo);
    impl_into_vecnod_request!(GetSyncStatus);
    impl_into_vecnod_request!(GetDaaScoreTimestampEstimate);
    impl_into_vecnod_request!(GetFeeEstimate);
    impl_into_vecnod_request!(GetFeeEstimateExperimental);
    impl_into_vecnod_request!(GetCurrentBlockColor);

    impl_into_vecnod_request!(NotifyBlockAdded);
    impl_into_vecnod_request!(NotifyNewBlockTemplate);
    impl_into_vecnod_request!(NotifyUtxosChanged);
    impl_into_vecnod_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_vecnod_request!(NotifyFinalityConflict);
    impl_into_vecnod_request!(NotifyVirtualDaaScoreChanged);
    impl_into_vecnod_request!(NotifyVirtualChainChanged);
    impl_into_vecnod_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_vecnod_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_vecnod_request_ex!(vecno_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_vecnod_request;

    macro_rules! impl_into_vecnod_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for vecnod_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for VecnodRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for vecnod_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for VecnodRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&vecnod_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &vecnod_request::Payload) -> RpcResult<Self> {
                    if let vecnod_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&VecnodRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &VecnodRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("VecnoRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for VecnodRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(vecnod_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for vecnod_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    vecnod_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_vecnod_request_ex;
}

pub mod vecnod_response_convert {
    use crate::protowire::*;
    use vecno_rpc_core::{RpcError, RpcResult};

    impl_into_vecnod_response!(Shutdown);
    impl_into_vecnod_response!(SubmitBlock);
    impl_into_vecnod_response!(GetBlockTemplate);
    impl_into_vecnod_response!(GetBlock);
    impl_into_vecnod_response!(GetInfo);
    impl_into_vecnod_response!(GetCurrentNetwork);

    impl_into_vecnod_response!(GetPeerAddresses);
    impl_into_vecnod_response!(GetSink);
    impl_into_vecnod_response!(GetMempoolEntry);
    impl_into_vecnod_response!(GetMempoolEntries);
    impl_into_vecnod_response!(GetConnectedPeerInfo);
    impl_into_vecnod_response!(AddPeer);
    impl_into_vecnod_response!(SubmitTransaction);
    impl_into_vecnod_response!(SubmitTransactionReplacement);
    impl_into_vecnod_response!(GetSubnetwork);
    impl_into_vecnod_response!(GetVirtualChainFromBlock);
    impl_into_vecnod_response!(GetBlocks);
    impl_into_vecnod_response!(GetBlockCount);
    impl_into_vecnod_response!(GetBlockDagInfo);
    impl_into_vecnod_response!(ResolveFinalityConflict);
    impl_into_vecnod_response!(GetHeaders);
    impl_into_vecnod_response!(GetUtxosByAddresses);
    impl_into_vecnod_response!(GetBalanceByAddress);
    impl_into_vecnod_response!(GetBalancesByAddresses);
    impl_into_vecnod_response!(GetSinkBlueScore);
    impl_into_vecnod_response!(Ban);
    impl_into_vecnod_response!(Unban);
    impl_into_vecnod_response!(EstimateNetworkHashesPerSecond);
    impl_into_vecnod_response!(GetMempoolEntriesByAddresses);
    impl_into_vecnod_response!(GetCoinSupply);
    impl_into_vecnod_response!(Ping);
    impl_into_vecnod_response!(GetMetrics);
    impl_into_vecnod_response!(GetConnections);
    impl_into_vecnod_response!(GetSystemInfo);
    impl_into_vecnod_response!(GetServerInfo);
    impl_into_vecnod_response!(GetSyncStatus);
    impl_into_vecnod_response!(GetDaaScoreTimestampEstimate);
    impl_into_vecnod_response!(GetFeeEstimate);
    impl_into_vecnod_response!(GetFeeEstimateExperimental);
    impl_into_vecnod_response!(GetCurrentBlockColor);

    impl_into_vecnod_notify_response!(NotifyBlockAdded);
    impl_into_vecnod_notify_response!(NotifyNewBlockTemplate);
    impl_into_vecnod_notify_response!(NotifyUtxosChanged);
    impl_into_vecnod_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_vecnod_notify_response!(NotifyFinalityConflict);
    impl_into_vecnod_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_vecnod_notify_response!(NotifyVirtualChainChanged);
    impl_into_vecnod_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_vecnod_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_vecnod_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_vecnod_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_vecnod_response_ex!(vecno_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_vecnod_response_base!(vecno_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_vecnod_response;

    macro_rules! impl_into_vecnod_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for vecnod_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    vecnod_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for VecnodResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(vecnod_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_vecnod_response_base;

    macro_rules! impl_into_vecnod_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for vecnod_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    vecnod_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for VecnodResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for vecnod_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    vecnod_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for VecnodResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_vecnod_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&vecnod_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &vecnod_response::Payload) -> RpcResult<Self> {
                    if let vecnod_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&VecnodResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &VecnodResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("VecnoResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_vecnod_response_ex;

    macro_rules! impl_into_vecnod_notify_response {
        ($name:tt) => {
            impl_into_vecnod_response!($name);

            paste::paste! {
                impl_into_vecnod_notify_response_ex!(vecno_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_vecnod_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_vecnod_notify_response_ex!(vecno_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_vecnod_notify_response;

    macro_rules! impl_into_vecnod_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_vecnod_notify_response_ex;
}
