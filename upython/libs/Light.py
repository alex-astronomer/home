import machine
import time
import math


class Light:
    ON = 1023
    OFF = 0

    def __init__(self):
        self.pin = machine.Pin(5)
        self.pwm = machine.PWM(self.pin, freq=500, duty=0)
        self.brightness = 1023

    def on(self):
        self.pwm.duty(self.brightness)

    def off(self):
        self.pwm.duty(self.OFF)

    def set_brightness(self, brightness_str):
        brightness = int(brightness_str)
        # convert input range 0-255 to output range 0-1023
        self.brightness = math.floor((((brightness - 0) * (1023 - 0)) / (255 - 0)) + 0)

    def blink(self, delay=0.250):
        self.pwm.duty(self.OFF)
        time.sleep(delay)
        self.pwm.duty(self.ON)
        time.sleep(delay)
        self.pwm.duty(self.OFF)
