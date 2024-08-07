import requests
import json

def main():
    # Creating a device
    with open('test/test_payload.json', 'r') as file:
        payload = json.load(file)

    headers = {'Content-Type': 'application/json'}

    response = requests.post('http://127.0.0.1:8080/api/post/new', data=json.dumps(payload), headers=headers)
    print(f'Status Code: {response.status_code}')
    print(f'Response Body: {response.text}')
    
    # Adding an entry
    payload = {
        "id": response.json()['id'],
        "sensor_name":"Temperature Sensor",
        "entry":{
            "value":"56.5",
            "time":"12:34:56"
        }
    }
    response = requests.post('http://127.0.0.1:8080/api/post/entry', data=json.dumps(payload), headers=headers)
    print(f'Status Code: {response.status_code}')
    print(f'Response Body: {response.text}')


if __name__ == '__main__':
    main()
