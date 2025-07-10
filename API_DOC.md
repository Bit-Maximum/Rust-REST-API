# 📡 API Documentation

[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/Bit-Maximum/rust-rest-api/blob/master/API_DOC.md)
[![ru](https://img.shields.io/badge/lang-ru-blue.svg)](https://github.com/Bit-Maximum/rust-rest-api/blob/master/translation/API_DOC.ru.md)

This document describes all available HTTP endpoints of the REST API for managing users, cities, roads, and calculating the shortest path between cities.

---

## 🏠 Home

- `GET /` — Connection check (returns "OK")

---

## 👤 Users

- `GET /api/v1/records?name=<name>`  
  Get all users matching a name pattern

- `GET /api/v1/records/:id`  
  Get a user by ID

- `POST /api/v1/records`  
  Add a new user
  **Request body example:**
  ```json
  {
    "name": "Ivan",
    "phone": "89001234567"
  }
  ```

- `PUT /api/v1/records/:id`  
  Update an existing user 
  **Request body example:**
  ```json
  {
    "id": 1,
    "name": "Ivan",
    "phone": "89001234567"
  }
  ```

- `DELETE /api/v1/records/:id`  
  Delete a user by ID

---
## 🏙️ Cities

- `GET /api/v1/cities`  
  Get a list of all cities

- `GET /api/v1/cities/:name`  
  Get a city by name

- `POST /api/v1/cities`  
  Add a new city
  **Request body example:**
  ```json
  {
    "name": "Moscow",
    "latitude": 55.7558,
    "longitude": 37.6173
  }
  ```

- `DELETE /api/v1/cities/:id`  
  Delete a city by ID

---

## 🛣️ Roads

- `GET /api/v1/roads`  
  Get a list of all roads

- `POST /api/v1/roads`  
  Add a new road
  **Request body example:**
  ```json
  {
    "city_a": 1,
    "city_b": 2,
    "length": 110
  }
  ```

- `DELETE /api/v1/roads/:id`  
  Delete a road by ID

---

## 📍 Shortest Path

- `GET /api/v1/path?from=<city_A>&to=<city_B>`  
  Returns the shortest path between two cities using Dijkstra’s algorithm.
  **Response example:**
  ```json
  {
    "path": ["Moscow", "Tver", "Saint Petersburg"],
    "distance": 673
  }
  ```

## 🧪 Testing
* The `test/` folder contains a Python script using the `requests` library
* The script automatically tests:
    * Adding cities and roads
    * Adding users
    * Route calculation
    * Object deletion
