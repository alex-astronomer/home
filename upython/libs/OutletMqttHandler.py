import time
from umqtt.simple import MQTTClient


class OutletMqttHandler:
    client = None
    outlet = None
    name = None

    def __init__(self, outlet, name):
        self.name = name
        self.client = MQTTClient(
            client_id=self.name,
            server="10.0.0.40",
            port=1883,
            user="alex",
            password="assblood",
            keepalive=10,
        )
        self.outlet = outlet
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
        self.client.publish("{}/available".format(self.name), "1", True)
        for suffix in [""]:
            self.client.subscribe("{}{}".format(self.name, suffix))

    def send_state(self):
        """
        Send state of the lightbulb to the MQTT server.

        """
        for suffix, state in self.outlet.state.items():
            self.client.publish("{}/{}".format(self.name, suffix), state)

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
        if topic_str == self.name:
            if message_str == "ON":
                self.outlet.on()
            elif message_str == "OFF":
                self.outlet.off()
        self.send_state()



