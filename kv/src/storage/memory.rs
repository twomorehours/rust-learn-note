use dashmap::{mapref::one::Ref, DashMap};

use crate::{Kvpair, Storage, Value};

#[derive(Debug, Default)]
pub struct Memtable {
    table: DashMap<String, DashMap<String, Value>>,
}

impl Memtable {
    fn get_or_create_table(&self, table: &str) -> Ref<String, DashMap<String, Value>> {
        match self.table.get(table) {
            Some(table) => table,
            None => {
                let entry = self.table.entry(table.to_string()).or_default();
                entry.downgrade()
            }
        }
    }

    pub fn new() -> Memtable {
        Memtable::default()
    }
}

impl Storage for Memtable {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.clone()))
    }

    fn set(
        &self,
        table: &str,
        key: String,
        value: crate::Value,
    ) -> Result<Option<crate::Value>, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value.into()))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|e| e.1))
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::Kvpair>, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table
            .iter()
            .map(|v| Kvpair::new(v.key().clone(), v.value().clone()))
            .collect())
    }

    fn get_iter(
        &self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, crate::KvError> {
        let table = self.get_or_create_table(table).clone();
        let iter = table.into_iter().map(|v| Kvpair::new(v.0, v.1));
        Ok(Box::new(iter))
    }
}
