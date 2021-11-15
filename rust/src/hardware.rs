// use std::process::Output;
// // use rppal::gpio::{Gpio, Error, OutputPin};

// pub struct LedController {
//     white: OutputPin,
// }

// impl LedController {
//     pub fn new() -> Result<LedController, Error> {
//         Ok(LedController {
//             white: Gpio::new()?.get(23)?.into_output()
//         })
//     }

//     pub fn on(&mut self) {
//         self.white.set_high();
//     }

//     pub fn off(&mut self) {
//         self.white.set_low();
//     }
// }
