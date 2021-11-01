import machine
from libs.utilities import normalize


class PinAK:
    def __init__(self, gpio_number):
        self.pin = machine.PWM(machine.Pin(gpio_number), freq=500, duty=0)
        self.raw_brightness = 0

    def set_desired_brightness(self, brightness):
        self.raw_brightness = normalize(int(brightness))

    def on(self):
        self.pin.duty(self.raw_brightness)

    def off(self):
        self.pin.duty(0)
