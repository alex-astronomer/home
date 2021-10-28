import machine
import time
import math


class Light:
    ON = 1023
    OFF = 0

    def __init__(self):
        self.pins = {
            key: {"pin": machine.PWM(machine.Pin(value), freq=500, duty=0), "brightness": 0}
            for key, value
            in {"white": 5, "red": 4, "green": 12, "blue": 14}.items()
        }

    def on(self):
        for color in self.pins:
            self.pins[color]["pin"].duty(self.pins[color]["brightness"])

    def off(self):
        for color in self.pins:
            self.pins[color]["pin"].duty(0)

    def set_brightness(self, brightness_str):
        for d in self.pins.values():
            d["brightness"] = 0
        self.pins["white"]["brightness"] = self.normalize_brightness(brightness_str)

    def set_rgb(self, red, green, blue):
        self.pins["white"]["brightness"] = 0
        self.pins["red"]["brightness"] = self.normalize_brightness(red)
        self.pins["green"]["brightness"] = self.normalize_brightness(green)
        self.pins["blue"]["brightness"] = self.normalize_brightness(blue)

    def normalize_brightness(self, brightness, old_max=255, new_max=1023):
        return math.floor(int(brightness) * (new_max / old_max))

    def blink(self, color="white", delay=0.250):
        self.pins[color]["pin"].duty(self.OFF)
        time.sleep(delay)
        self.pins[color]["pin"].duty(self.ON)
        time.sleep(delay)
        self.pins[color]["pin"].duty(self.OFF)
