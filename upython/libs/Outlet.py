from libs.pin.DigitalPin import DigitalPin
from libs.IDevice import IDevice


class Outlet(IDevice):
    def __init__(self):
        pin_mapping = {"blue": 13, "relay": 14}
        self.pins = {key: DigitalPin(value) for key, value in pin_mapping.items()}

    @property
    def state(self):
        return {
            "state": self.on_state,
        }

    def on(self):
        for pin in self.pins.values():
            pin.on()
        self.on_state = "ON"

    def off(self):
        for pin in self.pins.values():
            pin.off()
        self.on_state = "OFF"
