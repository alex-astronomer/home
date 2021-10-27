import machine
import network
import time
from umqtt.simple import MQTTClient
from libs.Light import Light
from libs.WifiHandler import WifiHandler
from libs.MqttHandler import MqttHandler


def main():
    light = Light()
    light.blink(delay=0.5)
    wifi = WifiHandler()
    mqtt = MqttHandler(light)
    light.blink(delay=0.5)
    while True:
        mqtt.client.check_msg()
        mqtt.client.ping()
        time.sleep(0.025)


if __name__ == '__main__':
    main()
