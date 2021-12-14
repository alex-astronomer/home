use rppal::gpio::{Gpio, OutputPin};

#[derive(PartialEq, Debug)]
pub enum OnState {
    On,
    Off,
}

pub struct DigitalPin {
    pin: OutputPin,
    on_state: OnState,
}

impl Pin for DigitalPin {
    fn new(pin_num: u8) -> Self {
        Self {
            pin: Gpio::new().unwrap().get(pin_num).unwrap().into_output(),
            on_state: OnState::Off,
        }
    }

    fn on(&mut self) {
        self.pin.set_high();
        self.on_state = OnState::On;
    }

    fn off(&mut self) {
        self.pin.set_low();
        self.on_state = OnState::Off;
    }
}

#[derive(Debug)]
pub struct Brightness {
    actual: u8,
    desired: u8,
}

impl Brightness {
    pub fn new() -> Self {
        Self {
            actual: 0,
            desired: 0,
        }
    }
}

#[derive(Debug)]
pub struct AnalogPin {
    pin: OutputPin,
    brightness: Brightness,
}

impl AnalogPin {
    pub fn get_desired_brightness(&self) -> u8 {
        self.brightness.desired
    }

    pub fn set_desired_brightness(&mut self, brightness: u8) {
        self.brightness.desired = brightness;
    }

    pub fn get_actual_brightness(&self) -> u8 {
        self.brightness.actual
    }

    fn expect_set_pwm_frequency(&mut self, brightness: u8) {
        let frequency = 1000.0;
        let duty_cycle = (1.0/255.0) * brightness as f64;
        self.pin.set_pwm_frequency(frequency, duty_cycle).expect("Could not set pwm frequency");
        self.brightness.actual = brightness;
    }
}

impl Pin for AnalogPin {
    fn new(pin_num: u8) -> Self {
        Self {
            pin: Gpio::new().unwrap().get(pin_num).unwrap().into_output(), 
            brightness: Brightness::new(),
        }
    }

    fn off(&mut self) {
        self.expect_set_pwm_frequency(0);
    }

    fn on(&mut self) {
        self.expect_set_pwm_frequency(self.brightness.desired);
    }
}

pub trait Pin {
    fn new(pin_num: u8) -> Self;
    fn on(&mut self);
    fn off(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use serial_test::serial;

    mod analog_pin {
        use super::*;

        #[rstest(input => [0, 123, 255])]
        #[serial]
        fn test_on(input: u8) {
            let mut test_pin = AnalogPin::new(27);
            test_pin.set_desired_brightness(input);
            test_pin.on();
            assert_eq!(test_pin.brightness.actual, input);
        }

        #[rstest(input => [0, 123, 255])]
        #[serial]
        fn test_off(input: u8) {
            let mut test_pin = AnalogPin::new(27);
            test_pin.set_desired_brightness(input);
            test_pin.on();
            test_pin.off();
            assert_eq!(test_pin.get_desired_brightness(), input);
            assert_eq!(test_pin.brightness.actual, 0);
        }

        #[rstest(input => [0, 123, 255])]
        #[serial]
        fn test_expect_set_pwm_frequency(input: u8) {
            let mut test_pin = AnalogPin::new(27);
            test_pin.expect_set_pwm_frequency(input);
            assert_eq!(test_pin.brightness.actual, input);
        }

        #[rstest(input => [0, 123, 255])]
        #[serial]
        fn test_desired_brightness(input: u8) {
            let mut test_pin = AnalogPin::new(27);
            test_pin.set_desired_brightness(input);
            assert_eq!(test_pin.get_desired_brightness(), input);
            assert_eq!(test_pin.brightness.desired, input);
            assert_eq!(test_pin.brightness.actual, 0);
        }
    }

    mod brightness {
        use super::*;

        #[rstest]
        fn test_new() {
            let b = Brightness::new();
            assert_eq!(b.actual, 0);
            assert_eq!(b.desired, 0);
        }
    }
    // TODO: add static for analogpin shared and digitalpin shared lazy static
    mod digital_pin {
        use super::*;

        #[rstest]
        #[serial]
        fn test_off() {
            let mut test_pin = DigitalPin::new(27);
            test_pin.off();
            assert_eq!(test_pin.on_state, OnState::Off);
        }

        #[rstest]
        #[serial]
        fn test_on() {
            let mut test_pin = DigitalPin::new(27);
            test_pin.on();
            assert_eq!(test_pin.on_state, OnState::On);
        }
    }
}
