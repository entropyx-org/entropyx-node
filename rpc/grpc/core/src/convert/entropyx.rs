use crate::protowire::{entropy_x_request, EntropyXRequest, EntropyXResponse};

impl From<entropy_x_request::Payload> for EntropyXRequest {
    fn from(item: entropy_x_request::Payload) -> Self {
        EntropyXRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<EntropyXRequest> for EntropyXRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<EntropyXResponse> for EntropyXResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod entropyx_request_convert {
    use crate::protowire::*;
    use entropyx_rpc_core::{RpcError, RpcResult};

    impl_into_entropyx_request!(Shutdown);
    impl_into_entropyx_request!(SubmitBlock);
    impl_into_entropyx_request!(GetBlockTemplate);
    impl_into_entropyx_request!(GetBlock);
    impl_into_entropyx_request!(GetInfo);

    impl_into_entropyx_request!(GetCurrentNetwork);
    impl_into_entropyx_request!(GetPeerAddresses);
    impl_into_entropyx_request!(GetSink);
    impl_into_entropyx_request!(GetMempoolEntry);
    impl_into_entropyx_request!(GetMempoolEntries);
    impl_into_entropyx_request!(GetConnectedPeerInfo);
    impl_into_entropyx_request!(AddPeer);
    impl_into_entropyx_request!(SubmitTransaction);
    impl_into_entropyx_request!(SubmitTransactionReplacement);
    impl_into_entropyx_request!(GetSubnetwork);
    impl_into_entropyx_request!(GetVirtualChainFromBlock);
    impl_into_entropyx_request!(GetBlocks);
    impl_into_entropyx_request!(GetBlockCount);
    impl_into_entropyx_request!(GetBlockDagInfo);
    impl_into_entropyx_request!(ResolveFinalityConflict);
    impl_into_entropyx_request!(GetHeaders);
    impl_into_entropyx_request!(GetUtxosByAddresses);
    impl_into_entropyx_request!(GetBalanceByAddress);
    impl_into_entropyx_request!(GetBalancesByAddresses);
    impl_into_entropyx_request!(GetSinkBlueScore);
    impl_into_entropyx_request!(Ban);
    impl_into_entropyx_request!(Unban);
    impl_into_entropyx_request!(EstimateNetworkHashesPerSecond);
    impl_into_entropyx_request!(GetMempoolEntriesByAddresses);
    impl_into_entropyx_request!(GetCoinSupply);
    impl_into_entropyx_request!(Ping);
    impl_into_entropyx_request!(GetMetrics);
    impl_into_entropyx_request!(GetConnections);
    impl_into_entropyx_request!(GetSystemInfo);
    impl_into_entropyx_request!(GetServerInfo);
    impl_into_entropyx_request!(GetSyncStatus);
    impl_into_entropyx_request!(GetDaaScoreTimestampEstimate);
    impl_into_entropyx_request!(GetFeeEstimate);
    impl_into_entropyx_request!(GetFeeEstimateExperimental);
    impl_into_entropyx_request!(GetCurrentBlockColor);

    impl_into_entropyx_request!(NotifyBlockAdded);
    impl_into_entropyx_request!(NotifyNewBlockTemplate);
    impl_into_entropyx_request!(NotifyUtxosChanged);
    impl_into_entropyx_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_entropyx_request!(NotifyFinalityConflict);
    impl_into_entropyx_request!(NotifyVirtualDaaScoreChanged);
    impl_into_entropyx_request!(NotifyVirtualChainChanged);
    impl_into_entropyx_request!(NotifySinkBlueScoreChanged);

    impl_into_entropyx_request!(GetTotalBurnedAmount); // ENX-CHANGE-BURN:
    impl_into_entropyx_request!(GetBurnRecordsByRange); // ENX-CHANGE-BURN:  
    impl_into_entropyx_request!(GetBurnRecordByUtxoId); // ENX-CHANGE-BURN:
    impl_into_entropyx_request!(GetCurrentBurnFee); // ENX-CHANGE-BURN:

    macro_rules! impl_into_entropyx_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_entropyx_request_ex!(entropyx_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_entropyx_request;

    macro_rules! impl_into_entropyx_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for entropy_x_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for EntropyXRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for entropy_x_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for EntropyXRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&entropy_x_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &entropy_x_request::Payload) -> RpcResult<Self> {
                    if let entropy_x_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&EntropyXRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &EntropyXRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("EntropyXRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for EntropyXRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(entropy_x_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for entropy_x_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    entropy_x_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_entropyx_request_ex;
}

pub mod entropyx_response_convert {
    use crate::protowire::*;
    use entropyx_rpc_core::{RpcError, RpcResult};

    impl_into_entropyx_response!(Shutdown);
    impl_into_entropyx_response!(SubmitBlock);
    impl_into_entropyx_response!(GetBlockTemplate);
    impl_into_entropyx_response!(GetBlock);
    impl_into_entropyx_response!(GetInfo);
    impl_into_entropyx_response!(GetCurrentNetwork);

    impl_into_entropyx_response!(GetPeerAddresses);
    impl_into_entropyx_response!(GetSink);
    impl_into_entropyx_response!(GetMempoolEntry);
    impl_into_entropyx_response!(GetMempoolEntries);
    impl_into_entropyx_response!(GetConnectedPeerInfo);
    impl_into_entropyx_response!(AddPeer);
    impl_into_entropyx_response!(SubmitTransaction);
    impl_into_entropyx_response!(SubmitTransactionReplacement);
    impl_into_entropyx_response!(GetSubnetwork);
    impl_into_entropyx_response!(GetVirtualChainFromBlock);
    impl_into_entropyx_response!(GetBlocks);
    impl_into_entropyx_response!(GetBlockCount);
    impl_into_entropyx_response!(GetBlockDagInfo);
    impl_into_entropyx_response!(ResolveFinalityConflict);
    impl_into_entropyx_response!(GetHeaders);
    impl_into_entropyx_response!(GetUtxosByAddresses);
    impl_into_entropyx_response!(GetBalanceByAddress);
    impl_into_entropyx_response!(GetBalancesByAddresses);
    impl_into_entropyx_response!(GetSinkBlueScore);
    impl_into_entropyx_response!(Ban);
    impl_into_entropyx_response!(Unban);
    impl_into_entropyx_response!(EstimateNetworkHashesPerSecond);
    impl_into_entropyx_response!(GetMempoolEntriesByAddresses);
    impl_into_entropyx_response!(GetCoinSupply);
    impl_into_entropyx_response!(Ping);
    impl_into_entropyx_response!(GetMetrics);
    impl_into_entropyx_response!(GetConnections);
    impl_into_entropyx_response!(GetSystemInfo);
    impl_into_entropyx_response!(GetServerInfo);
    impl_into_entropyx_response!(GetSyncStatus);
    impl_into_entropyx_response!(GetDaaScoreTimestampEstimate);
    impl_into_entropyx_response!(GetFeeEstimate);
    impl_into_entropyx_response!(GetFeeEstimateExperimental);
    impl_into_entropyx_response!(GetCurrentBlockColor);

    impl_into_entropyx_response!(GetTotalBurnedAmount); // ENX-CHANGE-BURN:
    impl_into_entropyx_response!(GetBurnRecordsByRange); // ENX-CHANGE-BURN:
    impl_into_entropyx_response!(GetBurnRecordByUtxoId); // ENX-CHANGE-BURN:
    impl_into_entropyx_response!(GetCurrentBurnFee); // ENX-CHANGE-BURN:

    impl_into_entropyx_notify_response!(NotifyBlockAdded);
    impl_into_entropyx_notify_response!(NotifyNewBlockTemplate);
    impl_into_entropyx_notify_response!(NotifyUtxosChanged);
    impl_into_entropyx_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_entropyx_notify_response!(NotifyFinalityConflict);
    impl_into_entropyx_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_entropyx_notify_response!(NotifyVirtualChainChanged);
    impl_into_entropyx_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_entropyx_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_entropyx_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_entropyx_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_entropyx_response_ex!(entropyx_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_entropyx_response_base!(entropyx_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_entropyx_response;

    macro_rules! impl_into_entropyx_response_base {
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

            impl From<$protowire_struct> for entropy_x_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    entropy_x_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for EntropyXResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(entropy_x_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_entropyx_response_base;

    macro_rules! impl_into_entropyx_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for entropy_x_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    entropy_x_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for EntropyXResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for entropy_x_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    entropy_x_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for EntropyXResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_entropyx_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&entropy_x_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &entropy_x_response::Payload) -> RpcResult<Self> {
                    if let entropy_x_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&EntropyXResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &EntropyXResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("EntropyXResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_entropyx_response_ex;

    macro_rules! impl_into_entropyx_notify_response {
        ($name:tt) => {
            impl_into_entropyx_response!($name);

            paste::paste! {
                impl_into_entropyx_notify_response_ex!(entropyx_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_entropyx_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_entropyx_notify_response_ex!(entropyx_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_entropyx_notify_response;

    macro_rules! impl_into_entropyx_notify_response_ex {
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
    use impl_into_entropyx_notify_response_ex;
}
