mod config;
mod models;

use models::client::Client;
use config::cnn;

use std::result::Result;
use mysql::prelude::Queryable;

fn main() -> Result<(), mysql::Error> {
    let mut cnn = cnn::get_connection()?;

    let clients = cnn.query_map(
        "SELECT id, name, phone FROM clients",
        |(id, name, phone)| {
            Client { id, name, phone }
        }
    )?;

    for client in clients {
        println!("{}", "-".repeat(60));
        println!("ID: {}", client.id);
        println!("Name: {}", client.name);
        println!("Phone: {}", client.phone);
    }
    println!("{}", "-".repeat(60));

    Ok(())
}
