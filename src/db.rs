use postgres::{Client, Error, IsolationLevel};
use std::sync::Mutex;
use ini::Ini;
use crate::models::*;


pub struct ConnectParams {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String
}


pub fn init_db(db: &mut Client) {
    db.execute(
        concat!(
        r#"CREATE TABLE IF NOT EXISTS person ("#,
        r#"id SERIAL PRIMARY KEY, "#,
        r#"name varchar(50), "#,
        r#"phone varchar(100))"#,
        ),
        &[]).unwrap();

    db.execute(
        concat!(
        r#"CREATE TABLE IF NOT EXISTS city ("#,
        r#"id SERIAL PRIMARY KEY, "#,
        r#"name varchar(50), "#,
        r#"latitude REAL,"#,
        r#"longitude REAL);"#,
        ),
        &[]).unwrap();

    db.execute(
        concat!(
        r#"CREATE TABLE IF NOT EXISTS road ("#,
        r#"id SERIAL PRIMARY KEY, "#,
        r#"city_a INTEGER, "#,
        r#"city_b INTEGER, "#,
        r#"length INTEGER, "#,
        r#"CONSTRAINT fk_city_a "#,
        r#"FOREIGN KEY(city_a) "#,
        r#"REFERENCES city(id), "#,
        r#"CONSTRAINT fk_city_b "#,
        r#"FOREIGN KEY(city_b) "#,
        r#"REFERENCES city(id)); "#,
        ),
        &[]).unwrap();
}


pub fn params() -> ConnectParams {
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
        password: password.parse().unwrap()
    }
}


pub fn insert_person(db: &mut Client, name: &str, phone: &str) -> Result<u64, Error> {
    db.execute("INSERT INTO person (name, phone) VALUES ($1, $2)",
               &[&name, &phone])
}


pub fn update_person(db: &mut Client, id: i32, name: &str, phone: &str) -> Result<(), Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::RepeatableRead)
        .start()?;

    transaction.execute("UPDATE person SET name = $1, phone = $2 WHERE id = $3",
                        &[&name, &phone, &id])?;

    transaction.commit()
}


pub fn remove_person(db: &mut Client, ids: &[i32]) -> Result<(), Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::ReadCommitted)
        .start()?;

    let stmt = transaction.prepare("DELETE FROM person WHERE id = $1")?;
    for id in ids{
        transaction.execute(&stmt, &[&id])?;
    }

    transaction.commit()
}


pub fn show_persons(db: &mut Client, arg: Option<&str>) -> Result<Vec<Person>, Error>{
    let template = match arg{
        Some(s) => format!("WHERE name LIKE '%{}%'", s),
        None => "".to_owned(),
    };

    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::RepeatableRead)
        .start()?;

    let stmt = transaction.prepare(&format!("SELECT * FROM person {} ORDER BY id", template))?;
    let rows = transaction.query(&stmt, &[])?;
    transaction.commit()?;

    let size = rows.iter().count();
    let mut results = Vec::with_capacity(size);
    for row in rows{
        let record = Person {
            id: row.get("id"),
            name: row.get("name"),
            phone: row.get("phone"),
        };
        results.push(record);
    }
    Ok(results)
}


pub fn format(records: &[Person]) {
    let max = records.iter().fold(0, |acc, ref item| {
        if item.name.chars().count() > acc {
            item.name.chars().count()
        } else {
            acc
        }
    });
    for v in records {
        println!("{:3?}    {1:2$}    {3}", v.id, v.name, max, v.phone);
    }
}


pub fn read(sync_db: &Mutex<Client>, name: Option<&str>) -> Result<Vec<Person>, ()> {
    if let Ok(records) = show_persons(&mut *sync_db.lock().unwrap(), name) {
        Ok(records)
    } else {
        Err(())
    }
}


pub fn read_one(sync_db: &Mutex<Client>, id: i32) -> Result<Person, ()> {
    let db = &mut *sync_db.lock().unwrap();
    let stmt = db.prepare("SELECT * FROM person WHERE id = $1").unwrap();
    if let Ok(rows) = db.query(&stmt, &[&id]) {
        let mut iter = rows.iter();
        if iter.len() != 1 {
            return Err(());
        }
        let row = iter.next().unwrap();
        let record = Person {
            id: row.get("id"),
            name: row.get("name"),
            phone: row.get("phone"),
        };

        Ok(record)
    } else {
        Err(())
    }
}

pub fn insert_city(db: &mut Client, name: &str, latitude: f32, longitude: f32) -> Result<u64, Error> {
    db.execute("INSERT INTO city (name, latitude, longitude) VALUES ($1, $2, $3)",
               &[&name, &latitude, &longitude])
}


pub fn get_cities(db: &mut Client) -> Result<Vec<City>, Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::RepeatableRead)
        .start()?;

    let stmt = transaction.prepare("SELECT * FROM city ORDER BY id")?;
    let rows = transaction.query(&stmt, &[])?;
    transaction.commit()?;

    let size = rows.iter().count();
    let mut results = Vec::with_capacity(size);
    for row in rows{
        let record = City {
            id: row.get("id"),
            name: row.get("name"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
        };
        results.push(record);
    }
    Ok(results)
}


pub fn get_city(sync_db: &Mutex<Client>, name: Option<&str>) -> Result<City, ()>{
    let db = &mut *sync_db.lock().unwrap();
    let stmt = db.prepare("SELECT * FROM city WHERE name = $1").unwrap();
    if let Ok(rows) = db.query(&stmt, &[&name]) {
        let mut iter = rows.iter();
        if iter.len() != 1 {
            return Err(());
        }
        let row = iter.next().unwrap();
        let record = City {
            id: row.get("id"),
            name: row.get("name"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
        };

        Ok(record)
    } else {
        Err(())
    }
}


pub fn remove_cities(db: &mut Client, ids: &[i32]) -> Result<(), Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::ReadCommitted)
        .start()?;

    let stmt = transaction.prepare("DELETE FROM city WHERE id = $1")?;
    for id in ids{
        transaction.execute(&stmt, &[&id])?;
    }

    transaction.commit()
}


pub fn insert_road(db: &mut Client, city_a: i32, city_b: i32, length: i32) -> Result<u64, Error> {
    db.execute("INSERT INTO road (city_a, city_b, length) VALUES ($1, $2, $3)",
               &[&city_a, &city_b, &length])
}


pub fn get_roads(db: &mut Client) -> Result<Vec<Road>, Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::RepeatableRead)
        .start()?;

    let stmt = transaction.prepare("SELECT * FROM road ORDER BY id")?;
    let rows = transaction.query(&stmt, &[])?;
    transaction.commit()?;

    let size = rows.iter().count();
    let mut results = Vec::with_capacity(size);
    for row in rows{
        let record = Road {
            id: row.get("id"),
            city_a: row.get("city_a"),
            city_b: row.get("city_b"),
            length: row.get("length"),
        };
        results.push(record);
    }
    Ok(results)
}


pub fn remove_roads(db: &mut Client, ids: &[i32]) -> Result<(), Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::ReadCommitted)
        .start()?;

    let stmt = transaction.prepare("DELETE FROM road WHERE id = $1")?;
    for id in ids{
        transaction.execute(&stmt, &[&id])?;
    }

    transaction.commit()
}
