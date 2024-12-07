use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: Option<i32>,
    pub name: String,
    pub phone: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Road {
    pub id: Option<i32>,
    pub city_a: i32,
    pub city_b: i32,
    pub length: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Railway {
    pub id: Option<i32>,
    pub city_a: i32,
    pub city_b: i32,
    pub length: i32
}