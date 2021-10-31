import requests
from bs4 import BeautifulSoup
import paho.mqtt.client as mqtt


def main():
    state = None
    gateway_url = 'http://10.0.0.1/'
    occupant_devices = ["AK", "iPhone"]

    # mqtt and session
    s = requests.Session()
    mqtt_client = mqtt.Client()
    mqtt_client.username_pw_set("alex", "assblood")
    mqtt_client.connect("10.0.0.40", 1883, keepalive=60)

    # login
    s.post(gateway_url + 'check.jst', data={"username": "admin", "password": "assblood"})
    while True:
        connected_devices_doc = s.get(gateway_url + 'connected_devices_computers.jst')
        soup = BeautifulSoup(connected_devices_doc.content)

        # get all the host names
        online_host_names = [
            u.text.strip()
            for r in soup.find_all(headers="host-name")
            for u in r.find_all('u')
        ]
        mqtt_client.publish("ak/alive", "i'm still here!")
        is_home = "AK" in online_host_names
        if is_home != state:
            state = is_home
            mqtt_client.publish("ak/ishome", state)
            print(state)

if __name__ == "__main__":
    main()
