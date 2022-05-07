use postgres::Client;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use self::cache::Table;

mod cache;

type Request = String;

pub fn run(mut db_client: Client, listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(&mut db_client, stream);
            }
            Err(err) => {
                println!("Connection failed: {err}");
            }
        }
    }
}

fn handle_connection(db_client: &mut Client, mut stream: TcpStream) {
    let mut buf = [0; 1024];
    while let Ok(n) = stream.read(&mut buf) {
        if let Some((tables, request)) =
            parse_message(String::from_utf8(buf[0..n].to_vec()).unwrap())
        {

        }
    }
}

fn parse_message(msg: String) -> Option<(Vec<Table>, Request)> {
    let components: Vec<&str> = msg.split(';').collect();
    if let Some((request, tables)) = components.split_last() {
        let tables = tables.iter().map(|s| s.to_string()).collect();
        let request = request.to_string();
        return Some((tables, request));
    }
    return None;
}
