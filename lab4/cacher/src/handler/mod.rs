use std::sync::mpsc::{Receiver, Sender};

use postgres::Client;

use crate::requests::{row_to_string, Request, Statement};

use self::cache::QueryCache;

mod cache;

pub fn run(
    mut client: Client,
    request_rx: Receiver<Request>,
    response_tx: Sender<String>,
    cache_cap: usize,
) {
    let mut cache = QueryCache::new(cache_cap);
    for request in request_rx {
        match request.statement {
            Statement::Select(statement) => {
                if let Some(entry) = cache.get_entry(&statement) {
                    response_tx.send(entry.clone());
                } else {
                    if let Ok(entry) = client.query(&statement, &[]) {
                        let entry: Vec<String> = entry
                            .iter()
                            .map(|elem| row_to_string(elem, request.entity.as_ref().unwrap()))
                            .collect();
                        let entry = entry.join("\n");
                        response_tx.send(entry);
                    } else {
                        panic!("BAD REQUEST")
                    }
                }
            }
            Statement::Update(statement) => {
                if let Ok(rows_modified) = client.execute(&statement, &[]) {
                    if rows_modified > 0 {
                        for table in request.tables {
                            cache.invalidate_table(&table);
                        }
                    }
                }
            }
            Statement::Delete(statement) => {
                if let Ok(rows_modified) = client.execute(&statement, &[]) {
                    if rows_modified > 0 {
                        for table in request.tables {
                            cache.invalidate_table(&table);
                        }
                    }
                }
            }
        }
    }
}
