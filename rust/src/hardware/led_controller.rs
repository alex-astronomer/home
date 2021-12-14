use std::{collections::HashMap};
use super::pins::Pin;

pub struct LedController<T: Pin> {
    pins: HashMap<String, T>,
    on: bool,
}

impl<T> LedController<T> where T: Pin {
    pub fn new() -> Self {
        LedController {
            pins: HashMap::new(),
            on: false,
        }
    }
    
    pub fn add_pins(&mut self, kv_pairs: Vec<(String, u8)>) {
        for (name, num) in kv_pairs {
            self.pins.insert(name, T::new(num));
        }
    }

    pub fn add_pin(&mut self, id: String, pin_num: u8) {
        self.pins.insert(id, T::new(pin_num));
    }

    pub fn off(&mut self) {
        self.pins.iter_mut().for_each(|p| p.1.off());
    }

    pub fn on(&mut self) {
        self.pins.iter_mut().for_each(|p| p.1.on());
    }

    pub fn pins(&self) -> &HashMap<String, T> {
        &self.pins
    }

    pub fn pins_mut(&mut self) -> &mut HashMap<String, T> {
        &mut self.pins
    }

    pub fn on_state(&self) -> bool {
        self.on
    }

    pub fn set_on_state(&mut self, on: bool) {
        self.on = on;
    }
}

// TODO: use add_pins in tests instead of the weird [5, 6].iter() stuff going on now
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::pins::AnalogPin;
    use rstest::{rstest, fixture};
    use serial_test::serial;

    #[fixture]
    fn led_controller() -> LedController<AnalogPin> {
        LedController::new()
    }

    #[rstest(input,
        case(true),
        case(false),
    )]
    fn test_on_state_accessors(
        mut led_controller: LedController<AnalogPin>,
        input: bool
    ) {
        led_controller.set_on_state(input);
        assert_eq!(led_controller.on_state(), input);
    }

    #[rstest]
    #[serial]
    fn test_pins(mut led_controller: LedController<AnalogPin>) {
        led_controller.add_pin("test".to_string(), 17);
        assert_eq!(led_controller.pins().len(), 1);
        led_controller.pins.clear();
        assert_eq!(led_controller.pins().len(), 0);
    }

    #[rstest]
    #[serial]
    fn test_pins_mut(mut led_controller: LedController<AnalogPin>) {
        led_controller.add_pin("test".to_string(), 17);
        let pins_mut = led_controller.pins_mut();
        assert_eq!(pins_mut.len(), 1);
        pins_mut.clear();
        assert_eq!(pins_mut.len(), 0);
    }

    #[rstest]
    fn test_on(mut led_controller: LedController<AnalogPin>) {
        // add 5 and 6 pins
        led_controller.add_pins(vec!(("asdf".to_string(), 5), ("qwer".to_string(), 6)));
        // set brightness for those pins to 255
        for pin in led_controller.pins_mut() {
            pin.1.set_desired_brightness(255);
        }
        // use the on command
        led_controller.on();
        // assert 5 and 6 have an actual brightness of 255
        for pin in led_controller.pins() {
            assert_eq!(pin.1.get_actual_brightness(), 255);
        }
        // remove the pins from the led controller to break down the test
        led_controller.pins_mut().clear();
    }

    #[rstest]
    fn test_off(mut led_controller: LedController<AnalogPin>) {
        // add 5 and 6 pins
        led_controller.add_pins(vec!(("asdf".to_string(), 5), ("qwer".to_string(), 6)));
        // set brightness for those pins to 255
        for pin in led_controller.pins_mut() {
            pin.1.set_desired_brightness(255);
        }
        // use the on command
        led_controller.on();
        // assert 5 and 6 have an actual brightness of 255
        for pin in led_controller.pins() {
            assert_eq!(pin.1.get_actual_brightness(), 255);
        }

        led_controller.off();

        for pin in led_controller.pins() {
            assert_eq!(pin.1.get_desired_brightness(), 255);
            assert_eq!(pin.1.get_actual_brightness(), 0);
        }
        // remove the pins from the led controller to break down the test
        led_controller.pins_mut().clear();
    }

    #[rstest]
    fn test_add_pins(mut led_controller: LedController<AnalogPin>) {
        assert_eq!(led_controller.pins().len(), 0);
        led_controller.add_pins(vec!(("asdf".to_string(), 5u8), ("qwer".to_string(), 6u8)));
        assert_eq!(led_controller.pins().len(), 2);
        led_controller.pins_mut().clear();
    }
}
