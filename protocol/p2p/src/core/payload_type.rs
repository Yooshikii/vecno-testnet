use crate::pb::vecnod_message::Payload as VecnodMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum VecnodMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&VecnodMessagePayload> for VecnodMessagePayloadType {
    fn from(payload: &VecnodMessagePayload) -> Self {
        match payload {
            VecnodMessagePayload::Addresses(_) => VecnodMessagePayloadType::Addresses,
            VecnodMessagePayload::Block(_) => VecnodMessagePayloadType::Block,
            VecnodMessagePayload::Transaction(_) => VecnodMessagePayloadType::Transaction,
            VecnodMessagePayload::BlockLocator(_) => VecnodMessagePayloadType::BlockLocator,
            VecnodMessagePayload::RequestAddresses(_) => VecnodMessagePayloadType::RequestAddresses,
            VecnodMessagePayload::RequestRelayBlocks(_) => VecnodMessagePayloadType::RequestRelayBlocks,
            VecnodMessagePayload::RequestTransactions(_) => VecnodMessagePayloadType::RequestTransactions,
            VecnodMessagePayload::IbdBlock(_) => VecnodMessagePayloadType::IbdBlock,
            VecnodMessagePayload::InvRelayBlock(_) => VecnodMessagePayloadType::InvRelayBlock,
            VecnodMessagePayload::InvTransactions(_) => VecnodMessagePayloadType::InvTransactions,
            VecnodMessagePayload::Ping(_) => VecnodMessagePayloadType::Ping,
            VecnodMessagePayload::Pong(_) => VecnodMessagePayloadType::Pong,
            VecnodMessagePayload::Verack(_) => VecnodMessagePayloadType::Verack,
            VecnodMessagePayload::Version(_) => VecnodMessagePayloadType::Version,
            VecnodMessagePayload::TransactionNotFound(_) => VecnodMessagePayloadType::TransactionNotFound,
            VecnodMessagePayload::Reject(_) => VecnodMessagePayloadType::Reject,
            VecnodMessagePayload::PruningPointUtxoSetChunk(_) => VecnodMessagePayloadType::PruningPointUtxoSetChunk,
            VecnodMessagePayload::RequestIbdBlocks(_) => VecnodMessagePayloadType::RequestIbdBlocks,
            VecnodMessagePayload::UnexpectedPruningPoint(_) => VecnodMessagePayloadType::UnexpectedPruningPoint,
            VecnodMessagePayload::IbdBlockLocator(_) => VecnodMessagePayloadType::IbdBlockLocator,
            VecnodMessagePayload::IbdBlockLocatorHighestHash(_) => VecnodMessagePayloadType::IbdBlockLocatorHighestHash,
            VecnodMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                VecnodMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            VecnodMessagePayload::DonePruningPointUtxoSetChunks(_) => VecnodMessagePayloadType::DonePruningPointUtxoSetChunks,
            VecnodMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                VecnodMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            VecnodMessagePayload::BlockWithTrustedData(_) => VecnodMessagePayloadType::BlockWithTrustedData,
            VecnodMessagePayload::DoneBlocksWithTrustedData(_) => VecnodMessagePayloadType::DoneBlocksWithTrustedData,
            VecnodMessagePayload::RequestPruningPointAndItsAnticone(_) => VecnodMessagePayloadType::RequestPruningPointAndItsAnticone,
            VecnodMessagePayload::BlockHeaders(_) => VecnodMessagePayloadType::BlockHeaders,
            VecnodMessagePayload::RequestNextHeaders(_) => VecnodMessagePayloadType::RequestNextHeaders,
            VecnodMessagePayload::DoneHeaders(_) => VecnodMessagePayloadType::DoneHeaders,
            VecnodMessagePayload::RequestPruningPointUtxoSet(_) => VecnodMessagePayloadType::RequestPruningPointUtxoSet,
            VecnodMessagePayload::RequestHeaders(_) => VecnodMessagePayloadType::RequestHeaders,
            VecnodMessagePayload::RequestBlockLocator(_) => VecnodMessagePayloadType::RequestBlockLocator,
            VecnodMessagePayload::PruningPoints(_) => VecnodMessagePayloadType::PruningPoints,
            VecnodMessagePayload::RequestPruningPointProof(_) => VecnodMessagePayloadType::RequestPruningPointProof,
            VecnodMessagePayload::PruningPointProof(_) => VecnodMessagePayloadType::PruningPointProof,
            VecnodMessagePayload::Ready(_) => VecnodMessagePayloadType::Ready,
            VecnodMessagePayload::BlockWithTrustedDataV4(_) => VecnodMessagePayloadType::BlockWithTrustedDataV4,
            VecnodMessagePayload::TrustedData(_) => VecnodMessagePayloadType::TrustedData,
            VecnodMessagePayload::RequestIbdChainBlockLocator(_) => VecnodMessagePayloadType::RequestIbdChainBlockLocator,
            VecnodMessagePayload::IbdChainBlockLocator(_) => VecnodMessagePayloadType::IbdChainBlockLocator,
            VecnodMessagePayload::RequestAntipast(_) => VecnodMessagePayloadType::RequestAntipast,
            VecnodMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                VecnodMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
