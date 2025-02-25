use std::{collections::HashMap, hash::Hash, sync::{Arc, LazyLock, Mutex}, time::{Duration, Instant}};

pub struct CacheItem<T> {
    pub time: Instant,
    pub value: T
}

type Cache<K, V> where K: Eq, K: Hash = Arc<Mutex<HashMap<K, CacheItem<V>>>>;

// Title, parent id
pub static MARVIN_PROJECT_CACHE: LazyLock<Cache<String, (String, String)>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub static MARVIN_LABEL_CACHE: LazyLock<Cache<String, String>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub static TOGGL_CLIENT_CACHE: LazyLock<Cache<String, i64>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub static TOGGL_PROJECT_CACHE: LazyLock<Cache<(i64, String), i64>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub static TOGGL_TASK_CACHE: LazyLock<Cache<(i64, String), i64>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub static TOGGL_TAG_CACHE: LazyLock<Cache<String, i64>> = LazyLock::new(|| {
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

pub fn cache_put<K, V>(cache: Cache<K, V>, key: K, value: V) where K: Eq, K: Hash, V: Clone {
    let now = Instant::now();
    let mut map = cache.lock().unwrap();
    map.insert(key, CacheItem {
        time: now,
        value
    });
}
