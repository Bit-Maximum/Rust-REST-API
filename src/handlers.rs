use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use postgres::Client;
use serde_json;

use std::io::Read;
use std::sync::Mutex;

use crate::db;
use crate::models::{City, Road, Person};
use crate::algorithm::*;




// Test connection
pub fn hello_world(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let mut db = sdb.lock().unwrap();
    let json_records;
    if let Ok(_) = db.query("SELECT schema_name FROM information_schema.schemata", &[]) {
        if let Ok(json) = serde_json::to_string(format!("Hello World! Your URL is {}", url).as_str()) {
            json_records = Some(json);
        } else {
            return Ok(Response::with((status::InternalServerError,
                                      "couldn't connect to database")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError,
                                        "couldn't find the database")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_records.unwrap())))
}

// Users
// Get all records that`s "name" argument match given template
pub fn get_records(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let mut name: Option<String> = None;
    let qp = url.query_pairs();
    if qp.count() != 1 {
        return Ok(Response::with((status::BadRequest,
                                  "passed more than one parameter or no parameters at all")));
    }
    let (key, value) = qp.last().unwrap();
    if key == "name" {
        name = Some(value.to_string());
    }

    let json_records;
    if let Ok(records) = db::read(sdb, name.as_ref().map(|s| &s[..])) {
        if let Ok(json) = serde_json::to_string(&records) {
            json_records = Some(json);
        } else {
            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read records from database")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_records.unwrap())))
}


// Get record with ID
pub fn get_record(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let path = url.path_segments().unwrap();
    let sid: &str = &path.last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    let json_record;
    if let Ok(recs) = db::read_one(sdb, id) {
        if let Ok(json) = serde_json::to_string(&recs) {
            json_record = Some(json);
        } else {
            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read records from database")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_record.unwrap())))
}


// Add new record from given JSON parameters
pub fn add_record(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    request.body.read_to_string(&mut body).unwrap();
    let decoded: serde_json::Result<Person> = serde_json::from_str(&body);
    if let Ok(record) = decoded {
        if record.name == "" || record.phone == "" {
            return Ok(Response::with((status::BadRequest, "empty name or phone")));
        }
        if let Ok(_) = db::insert_person(&mut *sdb.lock().unwrap(), &record.name, &record.phone) {
            Ok(Response::with(status::Created))
        } else {
            Ok(Response::with((status::InternalServerError, "couldn't insert record")))
        }
    } else {
        return Ok(Response::with((status::BadRequest, "couldn't decode JSON")))
    }
}


// Update record with give ID. Put data give in request:body <JSON>
pub fn update_record(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let path = url.path_segments().unwrap();
    let sid: &str = &path.last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    let mut body = String::new();
    request.body.read_to_string(&mut body).unwrap();
    let decoded: serde_json::Result<Person> = serde_json::from_str(&body);
    if let Ok(record) = decoded {
        if record.name == "" || record.phone == "" {
            return Ok(Response::with((status::BadRequest, "empty name or phone")));
        }
        if let Ok(_) = db::update_person(&mut *sdb.lock().unwrap(), id, &record.name, &record.phone) {
            Ok(Response::with(status::NoContent))
        } else {
            Ok(Response::with((status::NotFound, "couldn't update record")))
        }
    } else {
        return Ok(Response::with((status::BadRequest, "couldn't decode JSON")));
    }
}


// Delete record with given ID
pub fn delete_record(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let path = url.path_segments().unwrap();
    let sid: &str = &path.last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    if let Ok(_) = db::remove_person(&mut *sdb.lock().unwrap(), &[id]) {
        Ok(Response::with(status::NoContent))
    } else {
        Ok(Response::with((status::NotFound, "couldn't delete record")))
    }
}


// Cities
// Add new city from given JSON parameters
pub fn add_city(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    request.body.read_to_string(&mut body).unwrap();
    let decoded: serde_json::Result<City> = serde_json::from_str(&body);
    if let Ok(record) = decoded {
        if record.name == "" {
            return Ok(Response::with((status::BadRequest, "empty name")));
        }
        if let Ok(_) = db::insert_city(&mut *sdb.lock().unwrap(), &record.name, record.latitude, record.longitude) {
            Ok(Response::with(status::Created))
        } else {
            Ok(Response::with((status::InternalServerError, "couldn't insert record")))
        }
    } else {
        return Ok(Response::with((status::BadRequest, "couldn't decode JSON")))
    }
}

// Get all cities
pub fn get_cities(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let json_records;
    if let Ok(records) = db::get_cities(&mut *sdb.lock().unwrap()) {
        if let Ok(json) = serde_json::to_string(&records) {
            json_records = Some(json);
        } else {
            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read records from database")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_records.unwrap())))
}


// Get City with given name
pub fn get_city(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let path = url.path_segments().unwrap();
    let name = *&path.last();

    let json_record;
    if let Ok(recs) = db::get_city(sdb, name) {
        if let Ok(json) = serde_json::to_string(&recs) {
            json_record = Some(json);
        } else {
            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read cities from database")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_record.unwrap())))
}


// Delete city with given ID
pub fn delete_city(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let path = url.path_segments().unwrap();
    let sid: &str = &path.last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    if let Ok(_) = db::remove_cities(&mut *sdb.lock().unwrap(), &[id]) {
        Ok(Response::with(status::NoContent))
    } else {
        Ok(Response::with((status::NotFound, "couldn't delete record")))
    }
}


// Roads
// Add new road from given JSON parameters
pub fn add_road(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    request.body.read_to_string(&mut body).unwrap();
    let decoded: serde_json::Result<Road> = serde_json::from_str(&body);
    if let Ok(record) = decoded {
        if let Ok(_) = db::insert_road(&mut *sdb.lock().unwrap(), record.city_a, record.city_b, record.length) {
            Ok(Response::with(status::Created))
        } else {
            Ok(Response::with((status::InternalServerError, "couldn't insert record")))
        }
    } else {
        return Ok(Response::with((status::BadRequest, "couldn't decode JSON")))
    }
}

// Get all roads
pub fn get_roads(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let json_records;
    if let Ok(records) = db::get_roads(&mut *sdb.lock().unwrap()) {
        if let Ok(json) = serde_json::to_string(&records) {
            json_records = Some(json);
        } else {
            return Ok(Response::with((status::InternalServerError,
                                      "couldn't convert records to JSON")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError,
                                  "couldn't read records from database")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_records.unwrap())))
}


// Delete roads with given ID
pub fn delete_road(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let path = url.path_segments().unwrap();
    let sid: &str = &path.last().unwrap();
    let id;
    if let Ok(r) = sid.parse() {
        id = r;
    } else {
        return Ok(Response::with((status::BadRequest, "bad id")));
    }

    if let Ok(_) = db::remove_roads(&mut *sdb.lock().unwrap(), &[id]) {
        Ok(Response::with(status::NoContent))
    } else {
        Ok(Response::with((status::NotFound, "couldn't delete record")))
    }
}


pub fn get_shortest_path(sdb: &Mutex<Client>, request: &mut Request) -> IronResult<Response> {
    let url: url::Url = request.url.clone().into();
    let mut to_city: Option<String> = None;
    let mut from_city: Option<String> = None;
    let qp = url.query_pairs();
    if qp.count() != 2 {
        return Ok(Response::with((status::BadRequest,
                                  "passed more or less than two parameters")));
    }
    for (k, v) in qp {
        if k == "to" {
            to_city = Some(v.to_string());
        }
        if k == "from" {
            from_city = Some(v.to_string());
        }
    }
    if let Err(_) = db::get_city(sdb, Some(&*to_city.clone().unwrap())) {
        return Ok(Response::with((status::BadRequest, "Can`t found start city with given parameters")));
    }
    if let Err(_) = db::get_city(sdb, Some(&*from_city.clone().unwrap())) {
        return Ok(Response::with((status::BadRequest, "Can`t found destination city with given parameters")));
    }

    let mut json_records: Option<String> = None;
    let edges = db::get_roads(&mut *sdb.lock().unwrap());
    let nodes = db::get_cities(&mut *sdb.lock().unwrap());
    if let Ok(nodes) = nodes {
        if let Ok(edges) = edges {
            let graph = build_graph(nodes, edges);
            let path = dijkstra(from_city.clone().unwrap(), to_city.clone().unwrap(), graph);

            let cost: i32 = path.get(&*to_city.clone().unwrap()).unwrap().clone().1;
            let massage = format_path(from_city.unwrap(), to_city.unwrap(), &path);
            if let Ok(json) = serde_json::to_string(format!("{msg}\nPath length: {cost}",
                                                            msg=massage, cost=cost).as_str()) {
                json_records = Some(json);
            }
        } else {
            return Ok(Response::with((status::InternalServerError, "Couldn't get Roads data")));
        }
    } else {
        return Ok(Response::with((status::InternalServerError, "Can`t load Cities data.")));
    }
    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, json_records.unwrap())))
}



pub const ROUTS: &'static str = "Available routes:
Home:
    GET: / -> This page
Users records:
    [get, /api/v1/records/:id, handlers::get_record, get_record],
    [post, /api/v1/records, handlers::add_record, add_record],
    [put, /api/v1/records/:id, handlers::update_record, update_record],
    [delete, /api/v1/records/:id, handlers::delete_record, delete_record]
Cities records:
    [get, /api/v1/cities, handlers::get_cities, get_cities],
    [get, /api/v1/cities/:name, handlers::get_city, get_city],
    [post, /api/v1/cities, handlers::add_city, add_city],
    [delete, /api/v1/cities/:id, handlers::delete_city, delete_city]
Roads records:
    [get, /api/v1/roads, handlers::get_roads, get_roads],
    [post, /api/v1/roads, handlers::add_road, add_road],
    [delete, /api/v1/roads/:id, handlers::delete_road, delete_road]
Calculations && Algorithms:
    Get shortest path from one City to Another (by Dijkstra algorithm)
    [get, /api/v1/path, handlers::get_shortest_path, get_shortest_path]";
