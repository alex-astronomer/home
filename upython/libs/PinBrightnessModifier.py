from libs.PinAK import PinAK


class PinBrightnessModifier(PinAK):
    def __init__(self, gpio_number):
        super(PinBrightnessModifier, self).__init__(gpio_number)
        self.modifier = 0

    def set_modifier(self, brightness):
        self.modifier = int(brightness) / 255

    def on(self):
        self.pin.duty(round(self.raw_brightness * self.modifier))
