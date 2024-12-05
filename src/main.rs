extern crate postgres;
extern crate ini;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;

use postgres::{Client, NoTls};

mod db;
mod handlers;
mod commands;
mod models;

use commands::*;


fn main() {
    let params = db::params();
    let connection_string = format!("host={host} port={port}  dbname={dbname} user={user} password={password}",
        host=params.host,
        port=params.port,
        dbname=params.dbname,
        user=params.user,
        password=params.password
    );

    let mut db = Client::connect(&*connection_string, NoTls).unwrap();
    db::init_db(&mut db);

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => {
            match text.as_ref() {
                "add" => add(&mut db, &args),
                "del" => del(&mut db, &args),
                "edit" => edit(&mut db, &args),
                "show" => show(&mut db, &args),
                "help" => println!("{}", HELP),
                command => panic!("Invalid command: {}", command),
            }
        }
        None => serve(db),
    }
}
