use crate::model::stores::statuses::StatusesStoreReader;
use entropyx_consensus_core::blockstatus::BlockStatus;
use entropyx_database::prelude::StoreError;
use entropyx_hashes::Hash;
use parking_lot::RwLock;
use std::sync::Arc;

/// Multi-threaded block-statuses service imp
#[derive(Clone)]
pub struct MTStatusesService<T: StatusesStoreReader> {
    store: Arc<RwLock<T>>,
}

impl<T: StatusesStoreReader> MTStatusesService<T> {
    pub fn new(store: Arc<RwLock<T>>) -> Self {
        Self { store }
    }
}

impl<T: StatusesStoreReader> StatusesStoreReader for MTStatusesService<T> {
    fn get(&self, hash: Hash) -> Result<BlockStatus, StoreError> {
        self.store.read().get(hash)
    }

    fn has(&self, hash: Hash) -> Result<bool, StoreError> {
        self.store.read().has(hash)
    }
}
