import time
from umqtt.simple import MQTTClient


class MqttHandler:
    client = None
    light = None

    def __init__(self, light):
        self.client = MQTTClient(
            client_id="umqtt-dev",
            server="10.0.0.40",
            port=1883,
            user="alex",
            password="assblood",
            keepalive=10,
        )
        self.light = light
        self.client.set_callback(self.msg_callback)
        self.client.set_last_will("umqtt-dev/available", "0")
        self.connect()

    def connect(self):
        while self.client.connect() != 0:
            print(self.client.connect())
            print('not connected yet...')
            time.sleep(5)
        print('got connected to MQTT.')
        self.client.publish("umqtt-dev/available", "1", True)
        self.client.subscribe("umqtt-dev/#")

    def msg_callback(self, topic, message):
        topic_str = topic.decode()
        message_str = message.decode()
        print("got callback {} {}".format(topic_str, message_str))
        if topic_str == "umqtt-dev":
            if message_str == "ON":
                self.light.on()
            elif message_str == "OFF":
                self.light.off()
        elif topic_str == "umqtt-dev/brightness":
            self.light.set_brightness(message_str)
        elif topic_str == "umqtt-dev/rgb":
            red, green, blue = message_str.split(',')
            self.light.set_rgb(red, green, blue)



