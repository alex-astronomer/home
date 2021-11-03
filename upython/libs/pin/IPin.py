from abc import abstractmethod, ABC


class IPin(ABC):
    state = None

    @abstractmethod
    def on(self):
        pass

    @abstractmethod
    def off(self):
        pass
