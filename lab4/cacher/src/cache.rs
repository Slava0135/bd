use std::{collections::HashMap};
use lru::LruCache;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Query(String);
pub struct Data(String);
#[derive(Eq, Hash, PartialEq)]
pub struct Table(String);
pub struct QueryCache {
    storage: LruCache<Query, Data>,
    related_queries: HashMap<Table, Vec<Query>>
}

impl QueryCache {

    pub fn new(capacity: usize) -> Self {
        QueryCache {
            storage: LruCache::new(capacity),
            related_queries: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, query: Query, data: Data, used_tables: Vec<Table>) {
        for t in used_tables {
            if let Some(v) = self.related_queries.get_mut(&t) {
                v.push(query.clone());
            } else {
                self.related_queries.insert(t, vec![query.clone()]);
            }
        }
        self.storage.push(query, data);
    }

    pub fn get_entry(&mut self, query: Query) -> Option<&Data> {
        self.storage.get(&query)
    }

    pub fn invalidate_table(&mut self, table: Table) {
        if let Some(v) = self.related_queries.get_mut(&table) {
            for q in v.iter() {
                self.storage.pop(q);
            }
            v.clear();
        }
    }
}