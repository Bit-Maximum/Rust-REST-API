use std::sync::{Arc, Mutex};
use iron::*;
use postgres::Client;
use crate::{db, handlers};


// Macros
// Clone DB-Client MutexGuard to use it separately in different threads & routes
// Handlers need to own BD-Client to perform queries
macro_rules! register_handler {
    ($connection:ident, $router:ident.$method:ident, $route:expr,
     $handler:path, $route_id:expr) => {
        let connection = $connection.clone();
        $router.$method(
            $route,
            move |req: &mut Request|
            $handler(&*connection, req), $route_id);
    }
}


macro_rules! define_handlers {
    ($connection:ident, $router:ident,
     $( [$method:ident, $route:expr, $handler:path, $route_id:expr]),+ ) => {
        $( register_handler!($connection, $router.$method, $route, $handler, $route_id); )+
    }
}


pub fn serve(db: Client) {
    let sync_db = Arc::new(Mutex::new(db));
    let mut router = router::Router::new();

    define_handlers!(
        sync_db, router,
        [get, "/api/v1/records", handlers::get_records, "get_record"],
        [get, "/api/v1/records/:id", handlers::get_record, "get_record"],
        [post, "/api/v1/records", handlers::add_record, "add_record"],
        [put, "/api/v1/records/:id", handlers::update_record, "update_record"],
        [delete, "/api/v1/records/:id", handlers::delete_record, "delete_record"]
    );

    Iron::new(router).http("localhost:3000").unwrap();
}



pub fn add(mut db: &mut Client, args: &Vec<String>) {
    if args.len() != 4 {
        panic!("Usage: person add NAME PHONE");
    }
    let r = db::insert(&mut db, &args[2], &args[3])
        .unwrap();
    println!("{} rows affected", r);
}

pub fn del(mut db: &mut Client, args: &Vec<String>) {
    if args.len() < 3 {
        panic!("Usage: person del ID...");
    }
    let ids: Vec<i32> = args[2..].iter()
        .map(|s| s.parse().unwrap())
        .collect();

    db::remove(&mut db, &ids)
        .unwrap();
}

pub fn edit(mut db: &mut Client, args: &Vec<String>) {
    if args.len() != 5 {
        panic!("Usage: person edit ID NAME PHONE");
    }
    let id = args[2].parse().unwrap();
    db::update(&mut db, id, &args[3], &args[4])
        .unwrap();
}

pub fn show(mut db: &mut Client, args: &Vec<String>) {
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
}


pub const HELP: &'static str = "Usage: phonebook COMMAND [ARG]...
Commands:
    add NAME - create new record;
    del ID1 ID2... - delete record;
    edit ID        - edit record;
    show           - display all records;
    show STRING    - display records which contain a given substring in the name;
    serve          - start REST API server;
    help           - display this help.";


