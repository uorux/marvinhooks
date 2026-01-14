use std::{collections::HashMap, fmt::Debug, hash::Hash, sync::{Arc, LazyLock, Mutex}, time::{Duration, Instant}};

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

/// Log the current state of all Toggl caches
pub fn log_toggl_cache_state() {
    let clients = TOGGL_CLIENT_CACHE.lock().unwrap();
    let projects = TOGGL_PROJECT_CACHE.lock().unwrap();
    let tasks = TOGGL_TASK_CACHE.lock().unwrap();
    let tags = TOGGL_TAG_CACHE.lock().unwrap();

    println!("=== TOGGL CACHE STATE ===");
    println!("Clients ({} entries):", clients.len());
    for (name, item) in clients.iter() {
        println!("  '{}' -> {}", name, item.value);
    }
    println!("Projects ({} entries):", projects.len());
    for ((client_id, name), item) in projects.iter() {
        println!("  ({}, '{}') -> {}", client_id, name, item.value);
    }
    println!("Tasks ({} entries):", tasks.len());
    for ((project_id, name), item) in tasks.iter() {
        println!("  ({}, '{}') -> {}", project_id, name, item.value);
    }
    println!("Tags ({} entries):", tags.len());
    for (name, item) in tags.iter() {
        println!("  '{}' -> {}", name, item.value);
    }
    println!("=========================");
}

pub fn cache_get<K, V>(cache: Cache<K, V>, key: &K) -> Option<V> where K: Eq, K: Hash + Debug, V: Clone + Debug {
    let now = Instant::now();
    let map = cache.lock().unwrap();
    match map.get(&key) {
        Some(item) => {
            if now - item.time >= Duration::from_secs(60 * 5) {
                println!("[CACHE] GET {:?} -> EXPIRED", key);
                return None
            }
            println!("[CACHE] GET {:?} -> HIT: {:?}", key, item.value);
            Some(item.value.clone())
        }
        None => {
            println!("[CACHE] GET {:?} -> MISS", key);
            None
        }
    }
}

pub fn cache_put<K, V>(cache: Cache<K, V>, key: K, value: V) where K: Eq, K: Hash + Debug, V: Clone + Debug {
    println!("[CACHE] PUT {:?} -> {:?}", key, value);
    let now = Instant::now();
    let mut map = cache.lock().unwrap();
    map.insert(key, CacheItem {
        time: now,
        value
    });
}
