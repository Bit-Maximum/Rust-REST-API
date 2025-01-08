import random

import requests


def test_connection():
    r = requests.get('http://localhost:3000/')
    print(r.status_code)
    print(r.text)
    print('--Test connection--')

    r = requests.post('http://localhost:3000/api/v1/records', json={
        'name': 'Vlad',
        'phone': '24.4747'
    })
    print(r.status_code)
    print(r.text)

    r = requests.post('http://localhost:3000/api/v1/cities', json={
        'name': 'Vlad',
        'latitude': 24.4747,
        'longitude': 25.6363
    })
    print(r.status_code)
    print(r.text)


def load_data():
    # Test connection
    r = requests.get('http://localhost:3000/')
    print(r.status_code)
    print(r.text)
    print('--Test connection--')

    # Post request
    cities = [
        'Vladivostok',
        'Artem',
        'Nahodka',
        'Fokino',
        'Blagoveshensk',
        'Habarovsk',
        'Dalnegorsk',
        'To-Delete'
    ]
    for city in cities:
        r = requests.post('http://localhost:3000/api/v1/cities', json={
            'name': city,
            'latitude': 24.4747,
            'longitude': 25.6363
        })
    print(r.status_code)
    print('--Post Cities request--')

    # Delete request
    r = requests.delete(f'http://localhost:3000/api/v1/cities/{len(cities)}')
    print(r.status_code)
    print('--Delete request--')

    # Get all request
    r = requests.get('http://localhost:3000/api/v1/cities')
    print(r.status_code)
    print(r.text)
    print(len(r.json()))
    print('--Get all request--')

    # Add roads
    roads = [
        {
            'city_a': 1,
            'city_b': 2,
            'length': 15,
        },
        {
            'city_a': 2,
            'city_b': 3,
            'length': 12,
        },
        {
            'city_a': 3,
            'city_b': 4,
            'length': 26,
        },
        {
            'city_a': 1,
            'city_b': 4,
            'length': 100,
        },
        {
            'city_a': 1,
            'city_b': 5,
            'length': 30,
        },
        {
            'city_a': 1,
            'city_b': 7,
            'length': 60,
        },
        {
            'city_a': 5,
            'city_b': 7,
            'length': 46,
        },
        {
            'city_a': 5,
            'city_b': 6,
            'length': 14,
        },
        {
            'city_a': 6,
            'city_b': 7,
            'length': 21,
        }
    ]
    for road in roads:
        r = requests.post('http://localhost:3000/api/v1/roads', json=road)
    print(r.status_code)
    print('--Post roads--')


def get_path():
    r = requests.get('http://localhost:3000/')
    print(r.status_code)
    print(r.text)
    print('--Test connection--')


    r = requests.get('http://localhost:3000/api/v1/path', params={'to': 'Habarovsk', 'from': 'Vladivostok'})
    print(r.status_code)
    print(r.text.strip('"').replace('\\n', '\n'))
    print('--Test 1--')


    r = requests.get('http://localhost:3000/api/v1/path', params={'to': 'Fokino', 'from': 'Vladivostok'})
    print(r.status_code)
    print(r.text.strip('"').replace('\\n', '\n'))
    print('--Test 2--')


if __name__ == '__main__':
    test_connection()
    print("Connection trusted")

    load_data()
    print("Data loaded")

    get_path()
    print("Complete")
