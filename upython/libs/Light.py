import machine
import time
import math


class Light:
    ON = 1023
    OFF = 0
    state = "OFF"
    pins = {
        key: {"pin": machine.PWM(machine.Pin(value), freq=500, duty=0), "brightness": 0}
        for key, value
        in {"white": 5, "red": 4, "green": 12, "blue": 14}.items()
    }

    def on(self):
        """
        Turn light on and set state

        """
        for color in self.pins:
            self.pins[color]["pin"].duty(self.pins[color]["brightness"])
        self.state = "ON"

    def off(self):
        """
        Turn light off and set state

        """
        for color in self.pins:
            self.pins[color]["pin"].duty(0)
        self.state = "OFF"

    def set_brightness(self, brightness):
        """
        Set white brightness and turn off the RGB pins

        :param brightness: Brightness str 0-255
        :type brightness: Union[int, str]

        """
        for d in self.pins.values():
            d["brightness"] = 0
        # normalize the brightness from 0-255 -> 0-1023
        self.pins["white"]["brightness"] = self.normalize_brightness(brightness)

    def set_rgb(self, red, green, blue):
        """
        Set RGB brightness and turn off the white pin

        :param red: Brightness of red 0-255
        :type red: Union[int, str]
        :param green: Brightness of green 0-255
        :type green: Union[int, str]
        :param blue: Brightness of blue 0-255
        :type blue: Union[int, str]

        """
        self.pins["white"]["brightness"] = 0
        # normalize all brightnesses from 0-255 -> 0-1023
        self.pins["red"]["brightness"] = self.normalize_brightness(red)
        self.pins["green"]["brightness"] = self.normalize_brightness(green)
        self.pins["blue"]["brightness"] = self.normalize_brightness(blue)

    def normalize_brightness(self, brightness, old_max=255, new_max=1023):
        """
        Utility function to normalize

        :param brightness: Brightness value
        :type brightness: Union[str, int]
        :param old_max: Previous value for max, "converting from"
        :type old_max: int
        :param new_max: New value for max, "converting to"
        :type new_max: int
        :return: Normalized brightness
        :rtype: int

        """
        return math.floor(int(brightness) * (new_max / old_max))

    def get_state(self):
        """
        Get the current state of the lightbulb as a dict.

        :return: Light bulb state dict (key: state topic, value: state)
        :rtype: dict[str, str]

        """
        return {
            "state": self.state,
            "brightness/state": str(
                self.normalize_brightness(self.pins["white"]["brightness"], 1023, 255)
            ),
            "rgb/state": ",".join(
                [str(self.normalize_brightness(b, 1023, 255)) for b in [
                     self.pins["red"]["brightness"],
                     self.pins["green"]["brightness"],
                     self.pins["blue"]["brightness"],

                ]]
            )
        }

    def blink(self, color="white", delay=0.250):
        """
        Blink a light of param color for param delay (seconds)

        :param color: Color of light to blink [red, green, blue, white]
        :type color: str
        :param delay: Delay between on and off for the blink (in seconds)
        :type delay: int

        """
        self.pins[color]["pin"].duty(self.OFF)
        time.sleep(delay)
        self.pins[color]["pin"].duty(self.ON)
        time.sleep(delay)
        self.pins[color]["pin"].duty(self.OFF)
