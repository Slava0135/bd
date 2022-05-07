use postgres::{Client, NoTls, Error};

mod handler;

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres dbname=railway", NoTls)?;
    for row in client.query("SELECT * FROM stations", &[])? {
        let name: &str = row.get(1);
        let latitude: f32 = row.get(2);
        let longitude: f32 = row.get(3);
        println!("{name} {latitude} {longitude}");
    }
    return Ok(())
}
