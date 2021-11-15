use std::str::FromStr;
use thiserror::Error;
// use rppal::gpio::Gpio;
// use crate::hardware::LedController;

// IMPL
pub struct Light {
    name: String,
    mqtt_client: paho_mqtt::Client,
    // led_controller: LedController,
}

impl Light {
    pub fn new(name: String, mqtt_client: paho_mqtt::Client) -> Light {
        // Gpio::new()?;


        return 1.0;
        Light {
            name,
            mqtt_client,
            // led_controller: LedController::new()?
        }
    }

    pub fn handle_action(&self, action: paho_mqtt::Message) {
        if let Some(action_instruction) = Instruction::from_message(&action, self){
            match action_instruction {
                Instruction::Command(ref state) => self.command(&state),
                Instruction::Brightness(brightness) => self.brightness(brightness),
                Instruction::Rgb(ref rgb) => self.rgb(rgb),
            };
        }
    }

    fn command(&self, state: &CommandState) {
        // let gpio_acesss = Gpio::new().expect("couldn't init gpio access");
        // let mut white_pin = gpio_acesss.get(23).expect("couldn't init pin 32 gpio").into_output();
        // match state {
        //     CommandState::On => white_pin.set_high(),
        //     CommandState::Off => white_pin.set_low(),
        // };
        self.mqtt_client.publish(
            paho_mqtt::Message::new(
                format!("{}/state", self.name), state.to_string(), 0
            )
        ).expect(
            "On/Off state mismatch! Command state change failed to send. God speed, sir. *salutes*"
        );
    }

    fn brightness(&self, brightness: u8) {
        println!("Brightness set to {}", brightness);
        self.mqtt_client.publish(
            paho_mqtt::Message::new(
                format!("{}/state", self.name), brightness.to_string(), 0
            )
        ).expect(
            "Brightness state mismatch! Command state change failed to send. God speed, sir. *salutes*"
        );
    }

    fn rgb(&self, rgb: &Rgb) {
        println!("Rgb set to {:?}", rgb);
        self.mqtt_client.publish(
            paho_mqtt::Message::new(
                format!("{}/state", self.name), rgb.to_string(), 0
            )
        ).expect(
            "Rgb state mismatch! Command state change failed to send. God speed, sir. *salutes*"
        );
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Command(CommandState),
    Brightness(u8),
    Rgb(Rgb),
}

impl Instruction {
    pub fn from_message(msg: &paho_mqtt::Message, for_device: &Light) -> Option<Instruction> {
        let device_name = &for_device.name;
        match msg.topic() {
            command_topic if command_topic == device_name => {
                // TODO just convert command state to take from_str() From impl
                let maybe_state = CommandState::from_utf8(msg.payload()).unwrap_or(None);
                Some(Instruction::Command(maybe_state?))
            },
            command_topic if command_topic == format!("{}/brightness", device_name) => {
                let brightness_str = String::from_utf8(msg.payload().to_vec()).ok()?;
                let maybe_u8 = brightness_str.parse::<u8>().ok();
                Some(Instruction::Brightness(maybe_u8?))
            },
            command_topic if command_topic == format!("{}/rgb", device_name) => {
                let rgb_str = String::from_utf8(msg.payload().to_vec()).ok()?;
                Some(Instruction::Rgb(Rgb::from_str(&rgb_str).ok()?))
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum CommandState {
    On,
    Off,
}

impl CommandState {
    // TODO: implement From<&[u8]> trait here
    // TODO: convert to the regular function signature that String::from_utf8(bytes: &[u8]) uses
    fn from_utf8(bytes: &[u8]) -> Result<Option<CommandState>, std::string::FromUtf8Error> {
        let as_string = &String::from_utf8(bytes.to_vec())?[..];
        match as_string {
            "ON" => Ok(Some(CommandState::On)),
            "OFF" => Ok(Some(CommandState::Off)),
            _ => Ok(None),
        }
    }
}

impl std::fmt::Display for CommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandState::On => write!(f, "ON"),
            CommandState::Off => write!(f, "OFF"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rgb(u8, u8, u8);

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

impl std::str::FromStr for Rgb {
    type Err = RgbFromError;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        let str_split_iter: Vec<Result<u8, std::num::ParseIntError>> = from.split(',')
        .map(|n| n.parse::<u8>())
        .rev()
        .collect();

        str_split_iter.len();

        // assert valid structure or return error, 3 color values as a csv string -> "r,g,b"
        if let 3 = str_split_iter.len() { () } else { return Err(RgbFromError::InvalidStructure) };

        Ok(Rgb(
            str_split_iter[0].clone()?, 
            str_split_iter[1].clone()?,
            str_split_iter[2].clone()?,
        ))
    }
}

#[derive(Error, Debug, PartialEq)]
enum RgbFromError {
    #[error("rgb string initializer requires that there are 3 color u8 values separated by commas")]
    InvalidStructure,
    #[error("error while parsing one of the elements of the Rgb string to u8")]
    ParseIntError(#[from] std::num::ParseIntError),
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::*;
//
//     const INVALID_SEQUENCE: &[u8] = &[0xc3_u8, 0x28_u8];
//
//     mod rgb {
//         use super::*;
//         use std::num::IntErrorKind;
//
//         #[rstest(rgb, expected,
//             case::max(Rgb(255, 255, 255), "255,255,255"),
//             case::zeros(Rgb(0, 0, 0),"0,0,0"),
//             case::mid(Rgb(123, 123, 123),"123,123,123"),
//         )]
//         fn test_to_string(rgb: Rgb, expected: &str) {
//             assert_eq!(rgb.to_string(), expected);
//         }
//
//         #[rstest(from, expected,
//             case::max("255,255,255", Rgb(255, 255, 255)),
//             case::mid("100,100,100", Rgb(100, 100, 100)),
//             case::zeros("0,0,0", Rgb(0, 0, 0)),
//         )]
//         fn test_from_str_valid(from: &str, expected: Rgb) {
//             assert_eq!(Rgb::from_str(from).unwrap(), expected);
//         }
//
//         #[rstest(from, err,
//             case("0,0,256", &IntErrorKind::PosOverflow),
//             case("11568,256,0", &IntErrorKind::PosOverflow),
//             case("89454,256,121684", &IntErrorKind::PosOverflow),
//             case("0,0,-256", &IntErrorKind::InvalidDigit),
//             case("-1885,0,-8568", &IntErrorKind::InvalidDigit),
//             case("-89454,-256,-121684", &IntErrorKind::InvalidDigit),
//             case(",,,", &IntErrorKind::Empty),
//             case("asdf", &IntErrorKind::InvalidDigit),
//             case("", &IntErrorKind::Empty),
//         )]
//         fn test_from_str_error(from: &str, err: &std::num::IntErrorKind) {
//             let err_res = std::panic::catch_unwind( || Rgb::from_str(from) ).unwrap();
//             if let RgbFromError::ParseIntError(err_kind) = err_res.unwrap_err() {
//                 assert_eq!(
//                     err_kind.kind(),
//                     err
//                 );
//             }
//
//         }
//
//         #[rstest(from, err,
//
//             case("123,123", RgbFromError::InvalidStructure),
//             case("123", RgbFromError::InvalidStructure),
//             case("123,123,123,123", RgbFromError::InvalidStructure),
//         )]
//         fn test_from_str_error_oob(from: &str, err: RgbFromError) {
//             let err_res = std::panic::catch_unwind( || Rgb::from_str(from) ).unwrap();
//             assert_eq!(
//                 err_res.unwrap_err(),
//                 err
//             );
//         }
//     }
//
//     mod command_state {
//         use super::*;
//
//         #[rstest(input, expected,
//             case(CommandState::On, "ON"),
//             case(CommandState::Off, "OFF"),
//         )]
//         fn test_to_string(input: CommandState, expected: &str) {
//             assert_eq!(input.to_string(), expected);
//         }
//
//         #[rstest(input, expected,
//             case::on("ON".as_bytes(), Some(CommandState::On)),
//             case::off("OFF".as_bytes(), Some(CommandState::Off)),
//             case::invalid_command_but_parsable("ASDF".as_bytes(), None),
//         )]
//         fn test_from_utf8_valid(input: &[u8], expected: Option<CommandState>) {
//             assert_eq!(CommandState::from_utf8(input).unwrap(), expected);
//         }
//
//         #[rstest(input,
//             case::invalid_utf8_byte_sequence(INVALID_SEQUENCE),
//         )]
//         fn test_from_utf8_invalid(input: &[u8]) {
//             let err_res = std::panic::catch_unwind(|| CommandState::from_utf8(input)).unwrap();
//             assert_eq!(err_res.unwrap_err().into_bytes(), input);
//         }
//     }
//
//     mod instruction {
//         use super::*;
//         use paho_mqtt::{Client, CreateOptions, Message};
//         use const_format;
//
//         const DEVICE_NAME: &str = "test";
//         const BRIGHTNESS_TOPIC: &str = const_format::concatcp!(DEVICE_NAME, "/brightness");
//         const RGB_TOPIC: &str = const_format::concatcp!(DEVICE_NAME, "/rgb");
//
//         #[fixture]
//         fn for_device() -> Light {
//             Light::new(
//                 String::from_str(DEVICE_NAME).unwrap(),
//                 Client::new(CreateOptions::new()).unwrap(),
//             )
//         }
//
//         #[rstest(topic, payload, expected,
//             // command
//             case::command_on(
//                 DEVICE_NAME, "ON".as_bytes(), Some(Instruction::Command(CommandState::On))
//             ),
//             case::command_off(
//                 DEVICE_NAME, "OFF".as_bytes(), Some(Instruction::Command(CommandState::Off))
//             ),
//             // command invalid
//             case::command_invalid_message_valid_utf8(DEVICE_NAME, "invalid".as_bytes(), None),
//             case::command_invalid_utf8(DEVICE_NAME, INVALID_SEQUENCE, None),
//             // brightness invalid
//             case::brightness_pos_overflow(BRIGHTNESS_TOPIC, "256".as_bytes(), None),
//             case::brightness_invalid_digit(BRIGHTNESS_TOPIC, "-1".as_bytes(), None),
//             case::brightness_empty(BRIGHTNESS_TOPIC, "".as_bytes(), None),
//             case::brightness_invalid_sequence(BRIGHTNESS_TOPIC, INVALID_SEQUENCE, None),
//             case::brightness_invalid_digit(BRIGHTNESS_TOPIC, "asdf".as_bytes(), None),
//             // rgb valid
//             case::rgb_valid(
//                 RGB_TOPIC, "255,255,255".as_bytes(), Some(Instruction::Rgb(Rgb(255, 255, 255)))
//             ),
//             // rgb invalid
//             case::rgb_valid(RGB_TOPIC, "256,255,255".as_bytes(), None),
//             case::rgb_invalid_pos_overflow(RGB_TOPIC, "256,255,255".as_bytes(), None),
//             case::rgb_invalid_digit(RGB_TOPIC, "asdf,255,255".as_bytes(), None),
//             case::rgb_invalid_structure(RGB_TOPIC, "123,123,255,255".as_bytes(), None),
//         )]
//         fn test_from_message(
//             topic: &str,
//             payload: &[u8],
//             expected: Option<Instruction>,
//             for_device: Light
//         ) {
//             assert_eq!(
//                 Instruction::from_message(
//                     &Message::new(topic, payload, 0),
//                     &for_device
//                 ),
//                 expected
//             );
//         }
//
//         #[rstest]
//         fn test_from_message_brightness_valid_exhaustive(for_device: Light) {
//             for i in 0u8..255 {
//                 assert_eq!(
//                     Instruction::from_message(&Message::new(BRIGHTNESS_TOPIC, i.to_string(), 0), &for_device),
//                     Some(Instruction::Brightness(i))
//                 );
//             }
//         }
//     }
// }
