use postgres::{Client, Error, IsolationLevel};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use ini::Ini;
use crate::db;

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
}


pub fn params() -> ConnectParams {
    let conf = Ini::load_from_file("conf.ini").unwrap();
    let section = conf.section(Some("Connection")).unwrap();

    let host = section.get("host").unwrap();
    let port = section.get("port").unwrap();
    let dbname = section.get("dbname").unwrap();
    let user = section.get("user").unwrap();
    let password = section.get("password").unwrap();

    db::ConnectParams {
        host: host.parse().unwrap(),
        port: port.parse().unwrap(),
        dbname: dbname.parse().unwrap(),
        user: user.parse().unwrap(),
        password: password.parse().unwrap()
    }
}


pub fn insert(db: &mut Client, name: &str, phone: &str) -> Result<u64, Error> {
    db.execute("INSERT INTO person (name, phone) VALUES ($1, $2)",
               &[&name, &phone])
}


pub fn update(db: &mut Client, id: i32, name: &str, phone: &str) -> Result<(), Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::RepeatableRead)
        .start()?;

    transaction.execute("UPDATE person SET name = $1, phone = $2 WHERE id = $3",
                        &[&name, &phone, &id])?;

    transaction.commit()
}


pub fn remove(db: &mut Client, ids: &[i32]) -> Result<(), Error>{
    let mut transaction = db.build_transaction()
        .isolation_level(IsolationLevel::ReadCommitted)
        .start()?;

    let stmt = transaction.prepare("DELETE FROM person WHERE id = $1")?;
    for id in ids{
        transaction.execute(&stmt, &[&id])?;
    }

    transaction.commit()
}


pub fn show(db: &mut Client, arg: Option<&str>) -> Result<Vec<Record>, Error>{
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
        let record = Record{
            id: row.get("id"),
            name: row.get("name"),
            phone: row.get("phone"),
        };
        results.push(record);
    }
    Ok(results)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Record{
    id: Option<i32>,
    pub name: String,
    pub phone: String,
}


pub fn format(records: &[Record]) {
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


pub fn read(sync_db: &Mutex<Client>, name: Option<&str>) -> Result<Vec<Record>, ()> {
    if let Ok(records) = show(&mut *sync_db.lock().unwrap(), name) {
        Ok(records)
    } else {
        Err(())
    }
}


pub fn read_one(sync_db: &Mutex<Client>, id: i32) -> Result<Record, ()> {
    let db = &mut *sync_db.lock().unwrap();
    let stmt = db.prepare("SELECT * FROM person WHERE id = $1").unwrap();
    if let Ok(rows) = db.query(&stmt, &[&id]) {
        let mut iter = rows.iter();
        if iter.len() != 1 {
            return Err(());
        }
        let row = iter.next().unwrap();
        let record = Record {
            id: row.get("id"),
            name: row.get("name"),
            phone: row.get("phone"),
        };

        Ok(record)
    } else {
        Err(())
    }
}