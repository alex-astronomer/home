use rppal::gpio::{Gpio, OutputPin};
use crate::light::{Rgb, CommandState};
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct AnalogPin {
    pin: OutputPin,
    desired_brightness: u8,
}

impl AnalogPin {
    pub fn desired_brightness(&self) -> u8 {
        self.desired_brightness
    }
}

pub struct LedController {
    pins: [AnalogPin; 4],
    on: bool,
}

impl LedController {
    pub fn default() -> Self {
        LedController {
            pins: [
                AnalogPin {
                    pin: Gpio::new().unwrap().get(17).unwrap().into_output(),
                    desired_brightness: 0
                },
                AnalogPin {
                    pin: Gpio::new().unwrap().get(27).unwrap().into_output(),
                    desired_brightness: 0
                },
                AnalogPin {
                    pin: Gpio::new().unwrap().get(22).unwrap().into_output(),
                    desired_brightness: 0
                },
                AnalogPin {
                    pin: Gpio::new().unwrap().get(23).unwrap().into_output(),
                    desired_brightness: 0
                }
            ],
            on: false,
        }
    }

    pub fn on_state(&self) -> bool {
        self.on
    }

    pub fn pins(&self) -> &[AnalogPin; 4] {
        &self.pins
    }

    pub fn command(&mut self, on: bool){
        match on {
            true => self.on(),
            false => self.off(),
        };
        self.on = on;
    }

    pub fn brightness(&mut self, brightness: u8)  {
        self.pins[3].desired_brightness = brightness;
        for i in 0..3 {
            self.pins[i].desired_brightness = 0;
        }
    }

    pub fn rgb(&mut self, rgb: &Rgb) {
        let Rgb(red, green, blue) = rgb;
        for (i, color_brightness) in [*red, *green, *blue].iter().enumerate() {
            self.pins[i].desired_brightness = *color_brightness;
        }
        self.pins[3].desired_brightness = 0;
    }

    fn off(&mut self) -> [f64; 4] {
        self.pins
            .iter_mut()
            .map(|p| Self::expect_set_pwm_frequency(p, 0))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap()
    }

    fn on(&mut self) -> [f64; 4] {
        self.pins
            .iter_mut()
            .map(|p| Self::expect_set_pwm_frequency(p, p.desired_brightness))
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap()
    }

    fn expect_set_pwm_frequency(color_pin: &mut AnalogPin, brightness: u8) -> f64 {
        let frequency = 1000.0;
        let duty_cycle = (1.0/255.0) * brightness as f64;
        color_pin.pin.set_pwm_frequency(frequency, duty_cycle).expect("Could not set pwm frequency");
        duty_cycle
    }
}

lazy_static! {
    pub static ref LED_CONTROLLER: Mutex<LedController> = Mutex::new(LedController::default());
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;
    use float_cmp::{assert_approx_eq};

    #[rstest(input,
        case(true),
        case(false),
    )]
    fn test_command(input: bool) {
        let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
        led_controller_locked.command(input);
        assert_eq!(led_controller_locked.on, input);
    }
    // TODO: stdize the array equals stuff
    // TODO: make method that gets the desired brightness of all pins as an array maybe a calculated property?
    #[rstest]
    fn test_brightness_exhaustive() {
        for i in 0..=255 {
            let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
            led_controller_locked.rgb(&Rgb(255, 255, 255));
            led_controller_locked.brightness(i);
            let expected = [0, 0, 0, i];
            let actual: Vec<u8> = led_controller_locked.pins
                .iter()
                .map(|ap| ap.desired_brightness)
                .collect();
            println!("{:?}, {:?}", actual, expected);
            for (actual, expected) in actual.iter().zip(expected.iter()).collect::<Vec<(&u8, &u8)>>() {
                assert_eq!(actual, expected);
            }
        }
    }

    #[rstest(input, expected,
        case(Rgb(0, 0, 0), [0; 4]),
        case(Rgb(123, 123, 123), [123, 123, 123, 0]),
        case(Rgb(255, 255, 255), [255, 255, 255, 0]),
    )]
    fn test_rgb(input: Rgb, expected: [u8; 4]) {
        let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
        led_controller_locked.brightness(255);
        led_controller_locked.rgb(&input);
        let current_brightness: Vec<u8> = led_controller_locked.pins
            .iter()
            .map(|ap| ap.desired_brightness)
            .collect();
        println!("{:?}", current_brightness);
        for (actual, expected) in current_brightness.iter().zip(expected.iter()).collect::<Vec<(&u8, &u8)>>() {
            assert_eq!(actual, expected);
        }
    }

    #[rstest]
    fn test_on() {
        let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
        led_controller_locked.brightness(0);
        for (actual, expected) in led_controller_locked.on().iter().zip([0f64; 4].iter()).collect::<Vec<(&f64, &f64)>>() {
            assert_approx_eq!(f64, *actual, *expected, epsilon = 0.0001);
        }
        led_controller_locked.brightness(123);
        for (actual, expected) in led_controller_locked.on().iter().zip([0.0, 0.0, 0.0, 0.4823].iter()).collect::<Vec<(&f64, &f64)>>() {
            assert_approx_eq!(f64, *actual, *expected, epsilon = 0.0001);
        }
    }

    #[rstest]
    fn test_off() {
        let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
        let brightness = 123;
        led_controller_locked.brightness(brightness);
        for (a, e) in led_controller_locked.off().iter().zip([0f64; 4].iter()).collect::<Vec<(&f64, &f64)>>() {
            assert_approx_eq!(f64, *a, *e, epsilon = 0.0001);
        }
        assert_eq!(led_controller_locked.pins[3].desired_brightness, brightness);
    }

    #[rstest]
    fn test_expect_set_pwm_frequency_exhaustive() {
        let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
        for i in 0..=255 {
            LedController::expect_set_pwm_frequency(&mut led_controller_locked.pins[3], i);
        }
    }

    #[rstest(input, expected,
        case(0, 0.0),
        case(123, 0.4823),
        case(255, 1.0),
    )]
    fn test_expect_set_pwm_frequency_parallel_access_to_hardware(input: u8, expected: f64) {
        let mut led_controller_locked = LED_CONTROLLER.lock().unwrap();
        let duty = LedController::expect_set_pwm_frequency(&mut led_controller_locked.pins[3], input);
        println!("{} == {}", duty, expected);
        assert_approx_eq!(f64, duty, expected, epsilon = 0.0001);
    }
}
