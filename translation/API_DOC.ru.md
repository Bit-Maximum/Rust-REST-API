# 📡 Документация API

[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/Bit-Maximum/rust-rest-api/blob/master/API_DOC.md)
[![ru](https://img.shields.io/badge/lang-ru-blue.svg)](https://github.com/Bit-Maximum/rust-rest-api/blob/master/translation/API_DOC.ru.md)

Этот документ описывает все доступные HTTP-эндпоинты REST API для управления пользователями, городами, дорогами и расчёта кратчайшего пути между городами.

---

## 🏠 Главная

- `GET /` — Проверка подключения (возвращает "OK")

---

## 👤 Пользователи

- `GET /api/v1/records?name=<имя>`  
  Получить всех пользователей по шаблону имени

- `GET /api/v1/records/:id`  
  Получить пользователя по ID

- `POST /api/v1/records`  
  Добавить пользователя  
  **Пример тела запроса:**
  ```json
  {
    "name": "Иван",
    "phone": "89001234567"
  }
  ```

- `PUT /api/v1/records/:id`  
  Добавить пользователя  
  **Пример тела запроса:**
  ```json
  {
    "id": 1,
    "name": "Иван",
    "phone": "89001234567"
  }
  ```

- `DELETE /api/v1/records/:id`  
  Удалить пользователя по ID

---
## 🏙️ Города

- `GET /api/v1/cities`  
  Получить список всех городов

- `GET /api/v1/cities/:name`  
  Получить город по имени

- `POST /api/v1/cities`  
  Добавить новый город
  **Пример тела запроса:**
  ```json
  {
    "name": "Москва",
    "latitude": 55.7558,
    "longitude": 37.6173
  }
  ```

- `DELETE /api/v1/cities/:id`  
  Удалить город по ID

---

## 🛣️ Дороги

- `GET /api/v1/roads`  
  Получить список всех дорог

- `POST /api/v1/roads`  
  Добавить новую дорогу
  **Пример тела запроса:**
  ```json
  {
    "city_a": 1,
    "city_b": 2,
    "length": 110
  }
  ```

- `DELETE /api/v1/roads/:id`  
  Удалить дорогу по ID

---

## 📍 Кратчайший путь

- `GET /api/v1/path?from=<город_A>&to=<город_B>`  
  Возвращает кратчайший путь между двумя городами по алгоритму Дейкстры.
  **Пример ответа:**
  ```json
  {
    "path": ["Москва", "Тверь", "Санкт-Петербург"],
    "distance": 673
  }
  ```

## 🧪 Тестирование
* В папке `test/` находится Python-скрипт с использованием `requests`
* Автоматически проверяются:
  * Добавление городов, дорог 
  * Добавление пользователей 
  * Расчёт маршрутов 
  * Удаление объектов