use std::{sync::mpsc::channel, thread, time::Instant};

use handler::run_cached;
use postgres::{Client, NoTls, Error};

use crate::requests::{random_select, random_update_or_delete};

mod handler;
mod requests;

const cache_cap: usize = 8;
const select_chance: f32 = 0.9;
const n_requests: i32 = 100;

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres dbname=railway", NoTls)?;

    let (request_tx, request_rx) = channel();
    let (response_tx, response_rx) = channel();

    thread::spawn(move || {
        run_cached(client, request_rx, response_tx, cache_cap);
    });

    for i in 0..n_requests {
        let request = random_select();
        println!("{}", request.name);
        let start = Instant::now();
        request_tx.send(request);
        response_rx.recv();
        let duration = start.elapsed();
        println!("Time elapsed is: {duration:?}");
    }

    return Ok(())
}
