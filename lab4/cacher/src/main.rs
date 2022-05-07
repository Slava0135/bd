use std::{sync::mpsc::channel, thread};

use handler::run_cached;
use postgres::{Client, NoTls, Error};

use crate::requests::random_select;

mod handler;
mod requests;

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres dbname=railway", NoTls)?;

    let (request_tx, request_rx) = channel();
    let (response_tx, response_rx) = channel();

    thread::spawn(move || {
        run_cached(client, request_rx, response_tx, 8);
    });

    request_tx.send(random_select());
    let response = response_rx.recv().unwrap();
    println!("{response}");

    return Ok(())
}
