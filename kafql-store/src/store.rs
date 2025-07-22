use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub key: String,
    pub value: Option<serde_json::Value>,
    pub timestamp: u64,
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
}

#[derive(Debug, Clone)]
pub struct Store {
    data: Arc<DashMap<String, Record>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn upsert(&self, record: Record) {
        let key = record.key.clone();
        // Handle tombstoning - if value is None, we'll remove the record
        if record.value.is_none() {
            self.data.remove(&key);
            return;
        }
        // Always insert, replacing any existing value
        self.data.insert(key, record);
    }

    pub fn get(&self, key: &str) -> Option<Record> {
        self.data.get(key).map(|r| r.clone())
    }

    pub fn list(&self) -> Vec<Record> {
        self.data.iter().map(|r| r.clone()).collect()
    }

    pub fn create_tombstone(&self, key: &str, topic: &str, partition: i32, offset: i64) -> Record {
        Record {
            key: key.to_string(),
            value: None,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            topic: topic.to_string(),
            partition,
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_record(key: &str, value: Option<serde_json::Value>, timestamp: u64) -> Record {
        Record {
            key: key.to_string(),
            value,
            timestamp,
            topic: "test".to_string(),
            partition: 0,
            offset: 0,
        }
    }

    #[test]
    fn test_upsert_and_get() {
        let store = Store::new();
        let rec = sample_record("foo", Some(json!({"a": 1})), 1);
        store.upsert(rec.clone());
        let got = store.get("foo").unwrap();
        assert_eq!(got.key, "foo");
        assert_eq!(got.value, Some(json!({"a": 1})));
    }

    #[test]
    fn test_compaction() {
        let store = Store::new();
        let rec1 = sample_record("foo", Some(json!({"a": 1})), 1);
        let rec2 = sample_record("foo", Some(json!({"a": 2})), 2);
        store.upsert(rec1);
        store.upsert(rec2.clone());
        let got = store.get("foo").unwrap();
        assert_eq!(got.value, Some(json!({"a": 2})));
        assert_eq!(got.timestamp, 2);
    }

    #[test]
    fn test_tombstoning() {
        let store = Store::new();
        let rec = sample_record("foo", Some(json!({"a": 1})), 1);
        store.upsert(rec);
        let tombstone = sample_record("foo", None, 2);
        store.upsert(tombstone);
        assert!(store.get("foo").is_none());
    }

    #[test]
    fn test_list() {
        let store = Store::new();
        store.upsert(sample_record("foo", Some(json!({"a": 1})), 1));
        store.upsert(sample_record("bar", Some(json!({"b": 2})), 1));
        let mut keys: Vec<_> = store.list().into_iter().map(|r| r.key).collect();
        keys.sort();
        assert_eq!(keys, vec!["bar", "foo"]);
    }
}
