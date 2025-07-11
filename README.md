# 🦀 REST API for Logistics Routing in Rust

[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/Bit-Maximum/Rust-REST-API/blob/master/README.md)
[![ru](https://img.shields.io/badge/lang-ru-blue.svg)](https://github.com/Bit-Maximum/Rust-REST-API/blob/master/translation/README.ru.md)

This project is a REST API service written in **Rust** using the **Iron** framework and a **PostgreSQL** database. The service allows you to manage cities and the roads between them, as well as calculate the shortest path between two points using Dijkstra’s algorithm.

## 🚀 Features

- CRUD operations for users, cities, and roads
- Shortest path calculation between two cities
- Simple API interface with testing via Python script

## 📎 API Documentation

A full description of all endpoints and request formats is available in a separate file:

📄 [API Documentation →](./API_DOC.md)


## 🐳 Running with Docker

1. Clone the repository:
```bash
git clone https://github.com/Bit-Maximum/Rust-REST-API.git
cd Rust-REST-API
```
2. Build and run the project. Make sure `Docker` and `docker-compose` are installed. Then run:
```bash
docker-compose up --build
```
3. The API will be available at: `http://localhost:3000`


## 🛠️ (Alternative) Local Installation and Launch
> Make sure you have Rust and PostgreSQL installed, and the database is set up.
By default, the app connects using the settings from `conf.ini`:
> - host=postgres
> - port=5432
> - dbname=postgres
> - user=postgres
> - password=1234

1. Clone the repository:
```bash
git clone https://github.com/Bit-Maximum/Rust-REST-API.git
cd Rust-REST-API
```
2. Install dependencies:
```bash
cargo build
```
3. Run the server:
```bash
cargo run
```
4. The API will be available at: `http://localhost:3000`

## 📂 Testing
The `test/` folder contains a Python script that tests the API using the `requests` library. It automatically calls all the main API functions and checks their correctness.

## 📎 Notes
* This project was developed as a lab assignment for the course "Modern Programming Languages"
* The main goal is to demonstrate how to implement a REST API in Rust, interact with a database, and perform routing algorithm calculations.
