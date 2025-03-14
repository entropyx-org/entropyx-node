use crate::pb::entropy_x_message::Payload as EntropyXMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum EntropyXMessagePayloadType {
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

impl From<&EntropyXMessagePayload> for EntropyXMessagePayloadType {
    fn from(payload: &EntropyXMessagePayload) -> Self {
        match payload {
            EntropyXMessagePayload::Addresses(_) => EntropyXMessagePayloadType::Addresses,
            EntropyXMessagePayload::Block(_) => EntropyXMessagePayloadType::Block,
            EntropyXMessagePayload::Transaction(_) => EntropyXMessagePayloadType::Transaction,
            EntropyXMessagePayload::BlockLocator(_) => EntropyXMessagePayloadType::BlockLocator,
            EntropyXMessagePayload::RequestAddresses(_) => EntropyXMessagePayloadType::RequestAddresses,
            EntropyXMessagePayload::RequestRelayBlocks(_) => EntropyXMessagePayloadType::RequestRelayBlocks,
            EntropyXMessagePayload::RequestTransactions(_) => EntropyXMessagePayloadType::RequestTransactions,
            EntropyXMessagePayload::IbdBlock(_) => EntropyXMessagePayloadType::IbdBlock,
            EntropyXMessagePayload::InvRelayBlock(_) => EntropyXMessagePayloadType::InvRelayBlock,
            EntropyXMessagePayload::InvTransactions(_) => EntropyXMessagePayloadType::InvTransactions,
            EntropyXMessagePayload::Ping(_) => EntropyXMessagePayloadType::Ping,
            EntropyXMessagePayload::Pong(_) => EntropyXMessagePayloadType::Pong,
            EntropyXMessagePayload::Verack(_) => EntropyXMessagePayloadType::Verack,
            EntropyXMessagePayload::Version(_) => EntropyXMessagePayloadType::Version,
            EntropyXMessagePayload::TransactionNotFound(_) => EntropyXMessagePayloadType::TransactionNotFound,
            EntropyXMessagePayload::Reject(_) => EntropyXMessagePayloadType::Reject,
            EntropyXMessagePayload::PruningPointUtxoSetChunk(_) => EntropyXMessagePayloadType::PruningPointUtxoSetChunk,
            EntropyXMessagePayload::RequestIbdBlocks(_) => EntropyXMessagePayloadType::RequestIbdBlocks,
            EntropyXMessagePayload::UnexpectedPruningPoint(_) => EntropyXMessagePayloadType::UnexpectedPruningPoint,
            EntropyXMessagePayload::IbdBlockLocator(_) => EntropyXMessagePayloadType::IbdBlockLocator,
            EntropyXMessagePayload::IbdBlockLocatorHighestHash(_) => EntropyXMessagePayloadType::IbdBlockLocatorHighestHash,
            EntropyXMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                EntropyXMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            EntropyXMessagePayload::DonePruningPointUtxoSetChunks(_) => EntropyXMessagePayloadType::DonePruningPointUtxoSetChunks,
            EntropyXMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                EntropyXMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            EntropyXMessagePayload::BlockWithTrustedData(_) => EntropyXMessagePayloadType::BlockWithTrustedData,
            EntropyXMessagePayload::DoneBlocksWithTrustedData(_) => EntropyXMessagePayloadType::DoneBlocksWithTrustedData,
            EntropyXMessagePayload::RequestPruningPointAndItsAnticone(_) => EntropyXMessagePayloadType::RequestPruningPointAndItsAnticone,
            EntropyXMessagePayload::BlockHeaders(_) => EntropyXMessagePayloadType::BlockHeaders,
            EntropyXMessagePayload::RequestNextHeaders(_) => EntropyXMessagePayloadType::RequestNextHeaders,
            EntropyXMessagePayload::DoneHeaders(_) => EntropyXMessagePayloadType::DoneHeaders,
            EntropyXMessagePayload::RequestPruningPointUtxoSet(_) => EntropyXMessagePayloadType::RequestPruningPointUtxoSet,
            EntropyXMessagePayload::RequestHeaders(_) => EntropyXMessagePayloadType::RequestHeaders,
            EntropyXMessagePayload::RequestBlockLocator(_) => EntropyXMessagePayloadType::RequestBlockLocator,
            EntropyXMessagePayload::PruningPoints(_) => EntropyXMessagePayloadType::PruningPoints,
            EntropyXMessagePayload::RequestPruningPointProof(_) => EntropyXMessagePayloadType::RequestPruningPointProof,
            EntropyXMessagePayload::PruningPointProof(_) => EntropyXMessagePayloadType::PruningPointProof,
            EntropyXMessagePayload::Ready(_) => EntropyXMessagePayloadType::Ready,
            EntropyXMessagePayload::BlockWithTrustedDataV4(_) => EntropyXMessagePayloadType::BlockWithTrustedDataV4,
            EntropyXMessagePayload::TrustedData(_) => EntropyXMessagePayloadType::TrustedData,
            EntropyXMessagePayload::RequestIbdChainBlockLocator(_) => EntropyXMessagePayloadType::RequestIbdChainBlockLocator,
            EntropyXMessagePayload::IbdChainBlockLocator(_) => EntropyXMessagePayloadType::IbdChainBlockLocator,
            EntropyXMessagePayload::RequestAntipast(_) => EntropyXMessagePayloadType::RequestAntipast,
            EntropyXMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                EntropyXMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
