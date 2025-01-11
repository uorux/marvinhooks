use std::{collections::HashMap, hash::Hash, sync::{Arc, LazyLock, Mutex}, time::{Duration, Instant}};

use crate::models::tasks::ProjectOrCategory;

pub struct CacheItem<T> {
    pub time: Instant,
    pub value: T
}

type Cache<K, V> where K: Eq, K: Hash = Arc<Mutex<HashMap<K, CacheItem<V>>>>;

pub static MARVIN_PROJECT_CACHE: LazyLock<Cache<String, ProjectOrCategory>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});


pub fn cache_get<K, V>(cache: Cache<K, V>, key: &K) -> Option<V> where K: Eq, K: Hash, V: Clone {
    let now = Instant::now();
    let map = cache.lock().unwrap();
    match map.get(&key) {
        Some(item) => {
            if now - item.time >= Duration::from_secs(60 * 5) {
                return None
            }
            Some(item.value.clone())
        }
        None => None
    }
}

pub fn cache_put<K, V>(cache: Mutex<HashMap<K, CacheItem<V>>>, key: K, value: V) where K: Eq, K: Hash, V: Clone {
    let now = Instant::now();
    let mut map = cache.lock().unwrap();
    map.insert(key, CacheItem {
        time: now,
        value
    });
}
