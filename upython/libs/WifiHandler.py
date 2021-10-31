import network


class WifiHandler:
    wlan = network.WLAN(network.STA_IF)

    def __init__(self):
        self.connect()

    def connect(self):
        """
        Connect to WiFi and print connection details.

        """
        self.wlan.active(True)
        if not self.wlan.isconnected():
            print('Trying to connect to network...')
            self.wlan.connect('My Names Not Rick', 'sp0ngeb0b840!')
            while not self.wlan.isconnected():
                pass
        print("Connected to Wifi.")
        print('network config: ', self.wlan.ifconfig())
