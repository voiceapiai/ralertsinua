#![cfg(feature = "cache")]

use bytes::Bytes;
use quick_cache::sync::Cache;
use std::{fmt, sync::Arc};

use crate::ApiError;

type Result<T> = miette::Result<T, ApiError>;
type ApiCache = Cache<String, Bytes>;

/// Cache entry wrapper over `Bytes`
#[derive(Debug)]
pub struct CacheEntry(pub Bytes);

/// A trait providing methods for storing, reading, and removing cache records.
pub trait CacheManagerSync: Send + Sync + 'static {
    /// Attempts to pull a cached response and related last_modified from cache.
    fn get(&self, cache_key: &str) -> Result<Option<CacheEntry>>;
    /// Attempts to cache a response and related last_modified.
    fn put(&self, cache_key: &str, bytes: Bytes) -> Result<()>;
    /// Attempts to remove a record from cache.
    fn delete(&self, cache_key: &str) -> Result<()>;
}

/// Implements [`CacheManagerSync`] with [`quick-cache`](https://github.com/arthurprs/quick-cache) as the backend.
#[derive(Clone)]
pub struct CacheManagerQuick {
    /// The instance of `quick_cache::sync::Cache`
    pub cache: Arc<ApiCache>,
}

impl fmt::Debug for CacheManagerQuick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // need to add more data, anything helpful
        f.debug_struct("QuickManager").finish_non_exhaustive()
    }
}

impl CacheManagerQuick {
    /// Create a new manager from a pre-configured Cache
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Cache::new(capacity)),
        }
    }
}

impl CacheManagerSync for CacheManagerQuick {
    fn get(&self, cache_key: &str) -> Result<Option<CacheEntry>> {
        let entry: CacheEntry = match self.cache.get(cache_key) {
            Some(bytes) => CacheEntry(bytes),
            None => return Ok(None),
        };
        Ok(Some(entry))
    }

    fn put(&self, cache_key: &str, bytes: Bytes) -> Result<()> {
        self.cache.insert(cache_key.into(), bytes);
        Ok(())
    }

    fn delete(&self, cache_key: &str) -> Result<()> {
        self.cache.remove(cache_key);
        Ok(())
    }
}

// The existence of this function makes the compiler catch if the Buf
// trait is "object-safe" or not.
fn _assert_trait_object(_: &dyn CacheManagerSync) {}
