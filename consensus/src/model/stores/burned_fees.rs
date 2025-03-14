use entropyx_database::prelude::*;
use entropyx_database::registry::DatabaseStorePrefixes;
use entropyx_hashes::Hash;
use parking_lot::RwLock;
use rocksdb::WriteBatch;
use std::{collections::HashMap, sync::Arc}; // ENX-CHANGE-BURN:
use serde::{Deserialize, Serialize};
use entropyx_utils::mem_size::MemSizeEstimator;
use entropyx_core::{debug, info, warn};
use thiserror::Error;
use entropyx_consensus_core::burn::BurnRecord;

use super::U64BigKey;

/// ENX-CHANGE-BURN: 
/// The error of the burned fees store
#[derive(Error, Debug)]
pub enum BurnedFeesStoreError {
    #[error("Invalid DAA score: {0}")]
    InvalidDAAScore(u64),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(u64),

    #[error("Invalid DAA score range: {0} - {1}")]  
    InvalidDAAScoreRange(u64, u64),

    #[error("Store error: {0}")]
    StoreError(String),

    #[error("Rocksdb Error: {0}")]
    RocksdbError(String),
}

/// ENX-CHANGE-BURN: 
/// The index of the burned fees store
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DaaScoreIndex {
    block_hash: Hash,
    daa_score: u64,
}

impl MemSizeEstimator for DaaScoreIndex {
    fn estimate_mem_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DaaScoreIndexEntry(Arc<DaaScoreIndex>);

impl MemSizeEstimator for DaaScoreIndexEntry {
    fn estimate_mem_bytes(&self) -> usize {
        std::mem::size_of::<DaaScoreIndex>() + std::mem::size_of::<Self>()
    }
}

pub trait BurnedFeesStoreReader {
    fn get_burn_record(&self, block_hash: Hash) -> Result<Option<Arc<BurnRecord>>, BurnedFeesStoreError>;
    fn get_burn_records_by_range(&self, start_daa: u64, end_daa: u64) -> Result<Vec<Arc<BurnRecord>>, BurnedFeesStoreError>;
    fn get_latest_burn_record(&self) -> Result<Option<Arc<BurnRecord>>, BurnedFeesStoreError>;
}

pub trait BurnedFeesStore: BurnedFeesStoreReader {
    fn insert_burn_record(&self, record: BurnRecord) -> Result<(), BurnedFeesStoreError>;
}

// ENX-CHANGE-BURN:
pub trait TempBurnRecordStore {
    fn insert_temp_burn_record(&self, block_hash: Hash, burned_value: u64);
    fn get_temp_burn_record(&self, block_hash: &Hash) -> Option<u64>;
    fn remove_temp_burn_record(&self, block_hash: &Hash);
    fn clear_temp_records(&self);
}

/// ENX-CHANGE-BURN: 
/// The implementation of the burned fees store
#[derive(Clone)]
pub struct DbBurnedFeesStore {
    db: Arc<DB>,
    burn_records_access: CachedDbAccess<Hash, Arc<BurnRecord>>,
    daa_index_access: CachedDbAccess<U64BigKey, DaaScoreIndexEntry>,
    latest_daa_score: Arc<RwLock<u64>>,
    temp_records: Arc<RwLock<HashMap<Hash, u64>>>, // ENX-CHANGE-BURN:
}

impl DbBurnedFeesStore {
    pub fn new(db: Arc<DB>, burn_records_cache_policy: CachePolicy, daa_index_cache_policy: CachePolicy) -> Self {
        Self {
            db: db.clone(),
            burn_records_access: CachedDbAccess::new(
                db.clone(),
                burn_records_cache_policy,
                DatabaseStorePrefixes::BurnRecords.into(),
            ),
            daa_index_access: CachedDbAccess::new(
                db,
                daa_index_cache_policy,
                DatabaseStorePrefixes::BurnDaaIndex.into(),
            ),
            latest_daa_score: Arc::new(RwLock::new(0)),
            temp_records: Arc::new(RwLock::new(HashMap::new())), // ENX-CHANGE-BURN:
        }
    }

    pub fn init(&self) -> Result<(), BurnedFeesStoreError> {
        // Find the largest DAA score from the daa_index
        let latest_daa = self.find_latest_daa_score_from_db()?;
        
        let mut latest_score = self.latest_daa_score.write();
        *latest_score = latest_daa;
        
        debug!("[BURN]Initialized burn store with latest DAA score: {}", latest_daa);
        Ok(())
    }

    /// ENX-CHANGE-BURN: 
    /// Find the latest DAA score from the database
    fn find_latest_daa_score_from_db(&self) -> Result<u64, BurnedFeesStoreError> {
        // Use seek_last to get the latest DAA score
        match self.daa_index_access.seek_last() {
            Ok(Some((_, index_entry))) => Ok(index_entry.0.daa_score),
            Ok(None) => Ok(0),  // Return 0 when the database is empty
            Err(e) => Err(BurnedFeesStoreError::StoreError(e.to_string()))
        }
    }

    /// ENX-CHANGE-BURN: 
    /// Get the latest DAA score, prioritize the value in memory
    fn find_latest_daa_score(&self) -> Result<u64, BurnedFeesStoreError> {
        let current_score = self.get_latest_daa_score();
        if current_score > 0 {
            Ok(current_score)
        } else {
            self.find_latest_daa_score_from_db()
        }
    }

    /// ENX-CHANGE-BURN: 
    /// Atomic update the latest DAA score
    fn atomic_update_latest_daa_score(&self, new_score: u64) {
        let mut score = self.latest_daa_score.write();
        if new_score > *score {
            *score = new_score;
        }
    }

    /// ENX-CHANGE-BURN: 
    /// Get the latest DAA score
    fn get_latest_daa_score(&self) -> u64 {
        *self.latest_daa_score.read()
    }
}

impl BurnedFeesStoreReader for DbBurnedFeesStore {
    /// ENX-CHANGE-BURN: 
    /// Get the burn record by block hash
    fn get_burn_record(&self, block_hash: Hash) -> Result<Option<Arc<BurnRecord>>, BurnedFeesStoreError> {
        self.burn_records_access.read(block_hash)
            .map(Some)
            .or_else(|e| match e {
                StoreError::KeyNotFound(_) => Err(BurnedFeesStoreError::StoreError(String::from("Burn record hash key not found"))),
                e => Err(BurnedFeesStoreError::StoreError(e.to_string())),
            })
    }

    /// ENX-CHANGE-BURN: 
    /// Get the burn records by range
    fn get_burn_records_by_range(&self, start_daa: u64, end_daa: u64) -> Result<Vec<Arc<BurnRecord>>, BurnedFeesStoreError> {
        if start_daa > end_daa {
            return Err(BurnedFeesStoreError::InvalidDAAScoreRange(start_daa, end_daa));
        }

        debug!("[BURN]Begin to find burned records in range {} - {}", start_daa, end_daa);
        
        // Use seek_iterator to get all records in the range
        let range_size = std::cmp::min((end_daa - start_daa + 1) as usize, 200);
        let records = self.daa_index_access
            .seek_iterator(
                None,
                Some(U64BigKey::from(start_daa)),
                range_size,
                false
            )
            .take_while(|result| {
                // Check if it exceeds the range
                if let Ok((_, index_entry)) = result {
                    index_entry.0.daa_score <= end_daa
                } else {
                    true
                }
            })
            .filter_map(|result| result.ok())
            .filter_map(|(_, index_entry)| {
                self.burn_records_access
                    .read(index_entry.0.block_hash)
                    .ok()
            })
            .collect::<Vec<_>>();

        debug!("[BURN]Successfully find {} burned records in range {} - {}", 
            records.len(), start_daa, end_daa);
        
        Ok(records)
    }

    /// ENX-CHANGE-BURN: 
    /// Get the latest burn record
    fn get_latest_burn_record(&self) -> Result<Option<Arc<BurnRecord>>, BurnedFeesStoreError> {
        let latest_daa = self.find_latest_daa_score()?;
        if latest_daa == 0 {
            return Ok(None);
        }

        // Get the corresponding burn record
        self.daa_index_access
            .read(U64BigKey::from(latest_daa))
            .and_then(|index_entry| self.burn_records_access.read(index_entry.0.block_hash))
            .map(Some)
            .or_else(|e| match e {
                StoreError::KeyNotFound(_) => Ok(None),
                e => Err(BurnedFeesStoreError::StoreError(e.to_string())),
            })
    }
}

impl BurnedFeesStore for DbBurnedFeesStore {
    /// ENX-CHANGE-BURN: 
    /// Insert the burn record
    fn insert_burn_record(&self, record: BurnRecord) -> Result<(), BurnedFeesStoreError> {
        debug!("[BURN]Begin to insert burned record: hash={}, daa_score={}, burned={}, total_burned={}", 
            record.block_hash, record.block_daa_score, record.burned_value, record.total_burned);

        let mut batch = WriteBatch::default();
        
        let record_arc = Arc::new(record.clone());
        
        self.burn_records_access.write(
            BatchDbWriter::new(&mut batch),
            record.block_hash,
            record_arc,
        ).map_err(|e| {
            warn!("[BURN]Failed to write burned record: {}", e);
            BurnedFeesStoreError::StoreError(e.to_string())
        })?;

        self.daa_index_access.write(
            BatchDbWriter::new(&mut batch),
            U64BigKey::from(record.block_daa_score),
            DaaScoreIndexEntry(Arc::new(DaaScoreIndex {
                block_hash: record.block_hash,
                daa_score: record.block_daa_score,
            })),
        ).map_err(|e| {
            warn!("[BURN]Failed to write DAA index: {}", e);
            BurnedFeesStoreError::StoreError(e.to_string())
        })?;

        self.db.write(batch).map_err(|e| {
            warn!("[BURN]Failed to write to database: {}", e);
            BurnedFeesStoreError::RocksdbError(e.to_string())
        })?;
        
        self.atomic_update_latest_daa_score(record.block_daa_score);
        
        info!("[BURN]Successfully insert burned record: hash={}, daa_score={}, burned={}, total_burned={}", 
            record.block_hash, record.block_daa_score, record.burned_value, record.total_burned);
        
        Ok(())
    }
}

/// ENX-CHANGE-BURN: 
/// The implementation of the temporary burned fees store
impl TempBurnRecordStore for DbBurnedFeesStore {
    // insert temp burn record
    fn insert_temp_burn_record(&self, block_hash: Hash, burned_value: u64) {
        let mut temp_records = self.temp_records.write();
        temp_records.insert(block_hash, burned_value);
    }

    // get temp burn record
    fn get_temp_burn_record(&self, block_hash: &Hash) -> Option<u64> {
        let temp_records = self.temp_records.read();
        temp_records.get(block_hash).copied()
    }

    // remove temp burn record
    fn remove_temp_burn_record(&self, block_hash: &Hash) {
        let mut temp_records = self.temp_records.write();
        temp_records.remove(block_hash);
    }

    // clear all temp burn records
    fn clear_temp_records(&self) {
        let mut temp_records = self.temp_records.write();
        temp_records.clear();
    }
}
