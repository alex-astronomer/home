import time
from umqtt.simple import MQTTClient


class MqttHandler:
    client = None
    light = None
    name = "umqtt-dev"

    def __init__(self, light):
        self.client = MQTTClient(
            client_id=self.name,
            server="10.0.0.40",
            port=1883,
            user="alex",
            password="assblood",
            keepalive=10,
        )
        self.light = light
        self.client.set_callback(self.msg_callback)
        self.client.set_last_will("{}/available".format(self.name), "0")
        self.connect()

    def connect(self):
        """
        Connect to MQTT and subscribe to the required topics.

        """
        while self.client.connect() != 0:
            print(self.client.connect())
            print('not connected yet...')
            time.sleep(5)
        print('got connected to MQTT.')
        self.client.publish("umqtt-dev/available", "1", True)
        for suffix in ["", "/brightness", "/rgb", "/white"]:
            self.client.subscribe("{}{}".format(self.name, suffix))

    def send_state(self):
        """
        Send state of the lightbulb to the MQTT server.

        """
        for key, value in self.light.get_state().items():
            self.client.publish("umqtt-dev/{}".format(key), value)

    def msg_callback(self, topic, message):
        """
        Message callback function. Handles lightbulb actions based on
        message received and sends state back to HA.

        :param topic: byte-string of topic (must be decoded)
        :param message: byte-string of message received (must be decoded)

        """
        topic_str = topic.decode()
        message_str = message.decode()
        print("got callback {} {}".format(topic_str, message_str))
        if topic_str == "umqtt-dev":
            if message_str == "ON":
                self.light.on()
            elif message_str == "OFF":
                self.light.off()
        elif topic_str == "umqtt-dev/brightness":
            self.light.set_color_brightness(message_str)
        elif topic_str == "umqtt-dev/rgb":
            red, green, blue = message_str.split(',')
            self.light.set_rgb(red=red, green=green, blue=blue)
        elif topic_str == "umqtt-dev/white":
            self.light.set_white(message_str)
        self.send_state()



