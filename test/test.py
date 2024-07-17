import requests
import json

def main():
    # Creating a device
    with open('test/test_payload.json', 'r') as file:
        payload = json.load(file)

    headers = {'Content-Type': 'application/json'}

    response = requests.post('http://127.0.0.1:9090/devices/new', data=json.dumps(payload), headers=headers)
    print(f'Status Code: {response.status_code}')
    print(f'Response Body: {response.text}')
    
    # Adding an entry
    payload = {
        "device_id": response.json()['id'],
        "sensor_name":"Sensor1",
        "entry":{
            "value":"56.5",
            "time":"12:34:56"
        }
    }
    response = requests.post('http://127.0.0.1:9090/devices/post', data=json.dumps(payload), headers=headers)
    print(f'Status Code: {response.status_code}')
    print(f'Response Body: {response.text}')


if __name__ == '__main__':
    main()
