//! Implements a in-memory key-value store

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

/// A in-memory key value store
pub type InMemoryKeyValueStore<K, V> = Arc<Mutex<HashMap<K, V>>>;
