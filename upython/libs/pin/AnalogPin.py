import machine
from libs.utilities import normalize
from libs.pin.IPin import IPin


class AnalogPin(IPin):
    ON = 1023
    OFF = 0

    def __init__(self, gpio_number):
        self.pin = machine.PWM(machine.Pin(gpio_number), freq=500, duty=self.OFF)
        self.off()

    def set_state(self, new_state):
        self.state = normalize(int(new_state))

    def on(self):
        self.pin.duty(self.state)

    def off(self):
        self.pin.duty(self.OFF)
        self.state = self.OFF
