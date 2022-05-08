use std::{sync::mpsc::channel, thread, time::{Instant, Duration}, collections::HashMap, cmp::{min, max}};

use handler::{run_cached, run_uncached};
use postgres::{Client, NoTls};
use rand::Rng;

use crate::requests::{random_select, random_update_or_delete};

mod handler;
mod requests;

const cache_cap: usize = 8;
const select_chance: f32 = 0.8;
const n_requests: i32 = 10000;

struct RequestResults {
    min_time: Duration,
    max_time: Duration,
    total_time: Duration,
    n: u32,
}

type Results = HashMap<String, RequestResults>;

fn main() {
    test_cached();
    test_uncached();
}

fn test_cached() {
    
    let results = test(true);

    println!();
    println!("Results (Cached) with: cache_cap = {cache_cap}, select_chance = {select_chance}, n_requests = {n_requests}");
    println!();
    
    print_results(results);
}

fn test_uncached() {

    let results = test(false);

    println!();
    println!("Results (Uncached) with: select_chance = {select_chance}, n_requests = {n_requests}");
    println!();

    print_results(results);
}

fn test(cached: bool) -> Results {
    let client = Client::connect("host=localhost user=postgres dbname=railway", NoTls).unwrap();

    let (request_tx, request_rx) = channel();
    let (response_tx, response_rx) = channel();

    thread::spawn(move || {
        if cached {
            run_cached(client, request_rx, response_tx, cache_cap);
        } else {
            run_uncached(client, request_rx, response_tx);
        }
    });

    let mut rng = rand::thread_rng();
    let mut results: Results = HashMap::new();
    for i in 0..n_requests {
        let request = if rng.gen_range(0.0..1.0) > select_chance {
            random_update_or_delete()
        } else {
            random_select()
        };
        let request_name = request.name.clone();
        let start = Instant::now();
        request_tx.send(request);
        response_rx.recv();
        let duration = start.elapsed();
        if let Some(rr) = results.get_mut(request_name) {
            rr.min_time = min(rr.min_time, duration);
            rr.max_time = max(rr.max_time, duration);
            rr.total_time += duration;
            rr.n += 1;
        } else {
            results.insert(request_name.to_string(), RequestResults { min_time: duration, max_time: duration, total_time: duration, n: 1 });
        }
    }
    results
}

fn print_results(results: Results) {
    let mut keys: Vec<&String> = results.keys().collect();
    keys.sort();
    for key in keys {
        let v = results.get(key).unwrap();
        println!("{}:\n\tmin_time = {:?}\n\tmax_time = {:?}\n\tavg_time = {:?}", key, v.min_time, v.max_time, v.total_time / v.n);
    }
    println!();
    println!();
}