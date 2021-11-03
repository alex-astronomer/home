from abc import abstractmethod, ABC


class IDevice(ABC):
    on_state = "OFF"
    pins = None

    @property
    @abstractmethod
    def state(self):
        pass

    @abstractmethod
    def on(self):
        for pin in self.pins.values():
            pin.on()
        self.on_state = "ON"

    @abstractmethod
    def off(self):
        for pin in self.pins.values():
            pin.off()
        self.on_state = "OFF"
