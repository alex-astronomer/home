import time
from libs.Light import Light
from libs.WifiHandler import WifiHandler
from libs.MqttHandler import MqttHandler


def main():
    light = Light()
    wifi = WifiHandler()
    mqtt = MqttHandler(light, !name)
    last_ping = time.time()
    print(light.state)
    mqtt.send_state()
    while True:
        # loop through to check messages and every 8 seconds ping to keep the mqtt connection alive
        now = time.time()
        mqtt.client.check_msg()
        if now - last_ping > 8:
            mqtt.client.ping()
            last_ping = time.time()
        time.sleep(0.01)


if __name__ == '__main__':
    main()
