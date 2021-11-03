from libs.pin.AnalogPin import AnalogPin
from libs.utilities import normalize
from libs.pin.AnalogPinModifier import AnalogPinModifier
from libs.IDevice import IDevice


class Light(IDevice):
    def __init__(self):
        rgb_gpio_mapping = {"red": 4, "green": 12, "blue": 14}
        self.pins = {key: AnalogPinModifier(value) for key, value in rgb_gpio_mapping.items()}
        self.pins["white"] = AnalogPin(5)

    @property
    def state(self):
        """
        Get the current state of the lightbulb as a dict.

        :return: Light bulb state dict (key: state topic, value: state)
        :rtype: dict[str, str]

        """
        return {
            "state": self.on_state,
            "brightness/state": str(
                round(normalize(self.pins["white"].state, 1023, 255))
            ),
            "rgb/state": ",".join(
                [
                    str(round(normalize(b, 1023, 255))) for b in [
                        self.pins[color].modified_state for color in ["red", "green", "blue"]
                    ]
                ]
            )
        }

    def on(self):
        """
        Turn light on and set state

        """
        # combine pins into a single dict, since treating all as base class
        for pin in self.pins.values():
            pin.on()
        self.on_state = "ON"

    def off(self):
        """
        Turn light off and set state

        """
        # combine pins into a single dict, since treating all as base class
        for pin in self.pins.values():
            pin.off()
        self.on_state = "OFF"

    def set_white(self, white_value):
        """
        Set white brightness and turn off the RGB pins

        :param white_value: Brightness str 0-255
        :type white_value: Union[int, str]

        """
        self.pins["white"].set_state(white_value)
        for color in ["red", "green", "blue"]:
            self.pins[color].set_state(0)

    def set_color_brightness(self, brightness):
        """
        Set color brightness and turn off the white pin.

        :param brightness:
        :return:
        """
        self.pins["white"].set_state(0)
        for color in ["red", "green", "blue"]:
            self.pins[color].set_modifier(brightness)

    def set_rgb(self, **kwargs):
        """
        Set RGB full-range color values (not adjusted for brightness) and turn off the white pin

        :param red: Brightness of red 0-255
        :type red: Union[int, str]
        :param green: Brightness of green 0-255
        :type green: Union[int, str]
        :param blue: Brightness of blue 0-255
        :type blue: Union[int, str]

        """
        self.pins["white"].set_state(0)
        for color in ["red", "green", "blue"]:
            self.pins[color].set_state(kwargs[color])

    def blink(self, color="white", delay=0.250):
        """
        Blink a light of param color for param delay (seconds)

        :param color: Color of light to blink [red, green, blue, white]
        :type color: str
        :param delay: Delay between on and off for the blink (in seconds)
        :type delay: int

        """
        # self.rgbw[color]["pin"].on()
        # time.sleep(delay)
        # self.pins[color]["pin"].duty(self.ON)
        # time.sleep(delay)
        # self.pins[color]["pin"].duty(self.OFF)
