import machine
import time


class Light:
    pin = machine.Pin(5, machine.Pin.OUT)

    def on(self):
        self.pin.on()

    def off(self):
        self.pin.off()

    def blink(self, delay=0.250):
        self.pin.off()
        time.sleep(delay)
        self.pin.on()
        time.sleep(delay)
        self.pin.off()
