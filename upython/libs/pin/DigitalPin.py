import machine
from libs.pin.IPin import IPin


class DigitalPin(IPin):
    def __init__(self, gpio_number):
        self.pin = machine.Pin(gpio_number, machine.Pin.OUT)
        self.off()

    def on(self):
        self.pin.on()
        self.state = 1

    def off(self):
        self.pin.off()
        self.state = 0
