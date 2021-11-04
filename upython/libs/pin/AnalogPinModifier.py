from libs.pin.AnalogPin import AnalogPin


class AnalogPinModifier(AnalogPin):
    modifier = 0

    @property
    def modified_state(self):
        return round(self.state * self.modifier)

    def __init__(self, gpio_number):
        super(AnalogPinModifier, self).__init__(gpio_number)

    def set_modifier(self, new_modifier):
        self.modifier = int(new_modifier) / 255

    def on(self):
        self.pin.duty(self.modified_state)
