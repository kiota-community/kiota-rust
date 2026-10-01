//! A case-insensitive multi-value string map, used for HTTP headers.

use std::collections::HashMap;

/// A map that stores string keys in a case-insensitive manner (lowercased)
/// and supports multiple values per key.
///
/// This is primarily used for HTTP headers where header names are
/// case-insensitive per RFC 7230.
#[derive(Debug, Clone, Default)]
pub struct CaseInsensitiveMap {
    inner: HashMap<String, Vec<String>>,
}

impl CaseInsensitiveMap {
    /// Creates an empty `CaseInsensitiveMap`.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Appends a value under the given key (case-insensitive).
    ///
    /// If the key already exists, the value is appended to the existing list.
    pub fn insert(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let key = key.as_ref().to_lowercase();
        self.inner.entry(key).or_default().push(value.into());
    }

    /// Returns all values associated with the key, or `None` if the key is
    /// absent.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Vec<String>> {
        self.inner.get(&key.as_ref().to_lowercase())
    }

    /// Returns the first value associated with the key, or `None`.
    pub fn get_first(&self, key: impl AsRef<str>) -> Option<&String> {
        self.inner
            .get(&key.as_ref().to_lowercase())
            .and_then(|v| v.first())
    }

    /// Removes all values associated with the key.
    pub fn remove(&mut self, key: impl AsRef<str>) {
        self.inner.remove(&key.as_ref().to_lowercase());
    }

    /// Returns `true` if the map contains the given key (case-insensitive).
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner.contains_key(&key.as_ref().to_lowercase())
    }

    /// Returns an iterator over the stored (lowercased) keys.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.inner.keys()
    }

    /// Returns `true` if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the number of distinct keys in the map.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get_case_insensitive() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Content-Type", "application/json");
        assert_eq!(
            map.get("content-type").map(|v| v.as_slice()),
            Some(["application/json".to_string()].as_slice())
        );
        assert_eq!(
            map.get("CONTENT-TYPE").map(|v| v.as_slice()),
            Some(["application/json".to_string()].as_slice())
        );
    }

    #[test]
    fn insert_appends_multiple_values() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Accept", "text/html");
        map.insert("accept", "application/json");
        let values = map.get("Accept").unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "text/html");
        assert_eq!(values[1], "application/json");
    }

    #[test]
    fn get_first_returns_first_value() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("X-Custom", "first");
        map.insert("x-custom", "second");
        assert_eq!(map.get_first("X-CUSTOM"), Some(&"first".to_string()));
    }

    #[test]
    fn remove_deletes_key() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Authorization", "Bearer token");
        assert!(map.contains_key("authorization"));
        map.remove("AUTHORIZATION");
        assert!(!map.contains_key("authorization"));
    }

    #[test]
    fn contains_key_is_case_insensitive() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Host", "example.com");
        assert!(map.contains_key("host"));
        assert!(map.contains_key("HOST"));
        assert!(map.contains_key("Host"));
    }

    #[test]
    fn keys_returns_lowercased() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Content-Type", "text/plain");
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec!["content-type"]);
    }

    #[test]
    fn is_empty_and_len() {
        let mut map = CaseInsensitiveMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        map.insert("A", "1");
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let map = CaseInsensitiveMap::new();
        assert!(map.get("missing").is_none());
        assert!(map.get_first("missing").is_none());
    }
}
