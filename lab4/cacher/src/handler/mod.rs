use postgres::Client;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use self::cache::{QueryCache, Table};

mod cache;