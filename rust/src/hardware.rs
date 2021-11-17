use rppal::gpio::{Gpio, Error, OutputPin};
use crate::light::Rgb;

pub struct LedController {
    // TODO: extend output pin into a new struct that includes the brightness of that pin
    // TODO: put all pins into one rgbw array
    white: OutputPin,
    on: bool,
    brightness: u8,
    rgb: [OutputPin; 3],
    rgb_brightness: [u8; 3],
}

impl LedController {
    pub fn new() -> Result<LedController, Error> {
        let mut new = LedController::default()?;
        new.off();
        return Ok(new);
    }

    pub fn default() -> Result<LedController, Error> {
        Ok(LedController{
            white: Gpio::new()?.get(23)?.into_output(),
            on: false,
            brightness: 0,
            rgb: [
                Gpio::new()?.get(17)?.into_output(),
                Gpio::new()?.get(27)?.into_output(),
                Gpio::new()?.get(22)?.into_output(),
            ],
            rgb_brightness: [0, 0, 0],
        })
    }

    pub fn command(mut self, on: bool) -> Self {
        match on {
            true => self.on(),
            false => self.off(),
        };
        Self {
            on,
            ..self
        }
    }

    pub fn brightness(self, brightness: u8) -> Self {
        Self {
            brightness,
            rgb_brightness: [0, 0, 0],
            ..self
        }
    }

    pub fn rgb(self, rgb: &Rgb) -> Self {
        let Rgb(red, green, blue) = rgb;
        Self {
            brightness: 0,
            rgb_brightness: [*red, *green, *blue],
            ..self
        }
    }

    fn off(&mut self) {
        Self::expect_set_pwm_frequency(&mut self.white, 0);
        for i in 0..3 {
            Self::expect_set_pwm_frequency(&mut self.rgb[i], 0);
        }
    }

    fn on(&mut self) {
        Self::expect_set_pwm_frequency(&mut self.white, self.brightness);
        for i in 0..3 {
            Self::expect_set_pwm_frequency(&mut self.rgb[i], self.rgb_brightness[i]);
        }
    }

    fn expect_set_pwm_frequency(pin: &mut OutputPin, brightness: u8) {
        pin.set_pwm_frequency(
            1000f64, 
            (1.0/255 as f64) * brightness as f64
        ).expect("Could not set pwm frequency");
    }
}
