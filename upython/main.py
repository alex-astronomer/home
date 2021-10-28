import time
from libs.Light import Light
from libs.WifiHandler import WifiHandler
from libs.MqttHandler import MqttHandler


def main():
    light = Light()
    light.blink(delay=0.5)
    wifi = WifiHandler()
    mqtt = MqttHandler(light)
    light.blink(delay=0.5)
    last_ping = time.time()
    while True:
        now = time.time()
        mqtt.client.check_msg()
        if now - last_ping > 8:
            mqtt.client.ping()
            last_ping = time.time()
        time.sleep(0.01)


if __name__ == '__main__':
    main()
