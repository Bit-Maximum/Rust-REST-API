mod db;

extern crate postgres;
extern crate ini;
extern crate iron;
extern crate router;
extern crate url;

use ini::Ini;
use iron::*;
use postgres::{Client, NoTls};

use std::sync::{Mutex, Arc};

fn main() {
    let params = params();
    let connection_string = format!("host={host} port={port}  dbname={dbname} user={user} password={password}",
        host=params.host,
        port=params.port,
        dbname=params.dbname,
        user=params.user,
        password=params.password
    );

    let mut db = Client::connect(&*connection_string, NoTls).unwrap();
    init_db(&mut db);

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => {
            match text.as_ref() {
                "add" => {
                    if args.len() != 4 {
                        panic!("Usage: person add NAME PHONE");
                    }
                    let r = db::insert(&mut db, &args[2], &args[3])
                        .unwrap();
                    println!("{} rows affected", r);
                },
                "del" => {
                    if args.len() < 3 {
                        panic!("Usage: person del ID...");
                    }
                    let ids: Vec<i32> = args[2..].iter()
                        .map(|s| s.parse().unwrap())
                        .collect();

                    db::remove(&mut db, &ids)
                        .unwrap();
                },
                "edit" => {
                    if args.len() != 5 {
                        panic!("Usage: person edit ID NAME PHONE");
                    }
                    let id = args[2].parse().unwrap();
                    db::update(&mut db, id, &args[3], &args[4])
                        .unwrap();
                },
                "show" => {
                    if args.len() > 3 {
                        panic!("Usage: person show [SUBSTRING]");
                    }
                    let s;
                    if args.len() == 3 {
                        s = args.get(2);
                    } else {
                        s = None;
                    }
                    let r = db::show(&mut db, s.as_ref().map(|s| &s[..])).unwrap();
                    db::format(&r);
                },
                "serve" => {
                    let sync_db = Arc::new(Mutex::new(db));
                    let mut router = router::Router::new();
                    {
                        let sdb = sync_db.clone();
                        router.get("/api/v1/records",
                                   move |req: &mut Request|
                                       handlers::get_records(sdb.clone(), req));
                    }
                    {
                        let sdb = sync_db.clone();
                        router.get("/api/v1/records/:id",
                                   move |req: &mut Request|
                                       handlers::get_record(sdb.clone(), req));
                    }
                    {
                        let sdb = sync_db.clone();
                        router.post("/api/v1/records",
                                    move |req: &mut Request|
                                        handlers::add_record(sdb.clone(), req));
                    }
                    {
                        let sdb = sync_db.clone();
                        router.put("/api/v1/records/:id",
                                   move |req: &mut Request|
                                       handlers::update_record(sdb.clone(), req));
                    }
                    {
                        let sdb = sync_db.clone();
                        router.delete("/api/v1/records/:id",
                                      move |req: &mut Request|
                                          handlers::delete_record(sdb.clone(), req));
                    }
                    Iron::new(router).http("localhost:3000").unwrap();
                }
                "help" => {
                    println!("{}", HELP);
                },
                command => panic!("Invalid command: {}", command),
            }
        }
        None => panic!("No command supplied"),
    }
}


const HELP: &'static str = "Usage: phonebook COMMAND [ARG]...
Commands:
    add NAME - create new record;
    del ID1 ID2... - delete record;
    edit ID        - edit record;
    show           - display all records;
    show STRING    - display records which contain a given substring in the name;
    help           - display this help.";

struct ConnectParams {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    options: Option<Vec<u8>>
}


struct Person {
    id: i32,
    name: String,
    phone: String,
}


fn params() -> (ConnectParams) {
    let conf = Ini::load_from_file("conf.ini").unwrap();
    let section = conf.section(Some("Connection")).unwrap();

    let host = section.get("host").unwrap();
    let port = section.get("port").unwrap();
    let dbname = section.get("dbname").unwrap();
    let user = section.get("user").unwrap();
    let password = section.get("password").unwrap();

    ConnectParams {
        host: host.parse().unwrap(),
        port: port.parse().unwrap(),
        dbname: dbname.parse().unwrap(),
        user: user.parse().unwrap(),
        password: password.parse().unwrap(),
        options: None
    }
}


fn init_db(db: &mut Client) {
    db.execute(
        concat!(
            r#"CREATE TABLE IF NOT EXISTS person ("#,
            r#"id SERIAL PRIMARY KEY, "#,
            r#"name varchar(50), "#,
            r#"phone varchar(100))"#,
        ),
        &[]).unwrap();
}