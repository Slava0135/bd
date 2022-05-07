use postgres::{Client, NoTls, Error};
use requests::{row_to_string, Entity};

mod handler;
mod requests;

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres dbname=railway", NoTls)?;
    
    return Ok(())
}
