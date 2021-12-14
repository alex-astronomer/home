use std::str::FromStr;
use thiserror::Error;
use crate::common::{Controller, command_state::CommandState};
use crate::hardware::{pins::{AnalogPin}, led_controller::LedController};


type PinType = AnalogPin;

// IMPL
pub struct Light  {
    name: String,
    mqtt_client: paho_mqtt::Client,
    led_controller: LedController<PinType>,
}

impl Light {
    pub fn new(name: String, mqtt_client: paho_mqtt::Client) -> Light {
        Self {name, mqtt_client, led_controller: LedController::new()}
    }

    fn handle_brightness(&mut self, brightness: u8) {
        self.led_controller.pins_mut().get_mut("white").unwrap().set_desired_brightness(brightness);
        for pin in ["red", "green", "blue"] {
            self.led_controller.pins_mut().get_mut(pin).unwrap().set_desired_brightness(0);
        }
    }

    fn handle_rgb(&mut self, rgb: &Rgb) {
        let Rgb(red, green, blue) = rgb;
        self.led_controller.pins_mut().get_mut("white").unwrap().set_desired_brightness(0);
        for (color, desired_brightness) in [("red", red), ("green", green), ("blue", blue)] {
            self.led_controller.pins_mut().get_mut(color).unwrap().set_desired_brightness(*desired_brightness);
        }
    }

    fn get_rgb_state(&self) -> Rgb {
        Rgb(
            self.led_controller.pins()["red"].get_actual_brightness(), 
            self.led_controller.pins()["green"].get_actual_brightness(), 
            self.led_controller.pins()["blue"].get_actual_brightness()
        )
    }
}

impl Controller<PinType> for Light {
    type InstructionType = Instruction;

    fn default(name: String, mqtt_client: paho_mqtt::Client) -> Light {
        let mut new = Self {name, mqtt_client, led_controller: LedController::new()};
        new.led_controller.add_pins(vec!(
            ("red".to_string(), 17),
            ("green".to_string(), 27),
            ("blue".to_string(), 22),
            ("white".to_string(), 23)
        ));
        new
    }

    fn handle_action(&mut self, action: paho_mqtt::Message) {
        if let Some(instruction) = Instruction::from_message(&action, &self.name) {
            match &instruction {
                Instruction::Command(state) => self.handle_command(&state),
                Instruction::Brightness(brightness) => self.handle_brightness(*brightness),
                Instruction::Rgb(rgb) => self.handle_rgb(rgb),
            };
            // self.send_state(instruction, led_controller_locked);
            for state_message in self.get_states_to_send(instruction) {
                self.mqtt_client.publish(state_message).expect("mismatched state, see ya!");
            }
        }
    }

    fn get_states_to_send(&self, inst: Self::InstructionType) -> Vec<paho_mqtt::Message> {
        match inst {
            Instruction::Command(_) => {
                vec![paho_mqtt::Message::new(
                    format!("{}/state", &self.name), 
                    CommandState::from(self.led_controller.on_state()).to_string(), 
                    0
                )]
            },
            Instruction::Brightness(_) | Instruction::Rgb(_) => {
                vec![
                    paho_mqtt::Message::new(
                        format!("{}/brightness/state", &self.name), 
                        self.led_controller.pins()["white"].get_desired_brightness().to_string(), 
                        0
                    ),
                    paho_mqtt::Message::new(
                        format!("{}/rgb/state", &self.name), 
                        self.get_rgb_state().to_string(),
                        0
                    )
                ]
            }
        }
    }

    fn get_led_controller(&mut self) -> &mut LedController<PinType> {
        &mut self.led_controller
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Command(CommandState),
    Brightness(u8),
    Rgb(Rgb),
}

impl Instruction {
    // TODO: convert into From trait for message
    // TODO: find out if we need a separate from trait for a reference of something that we want to convert from
    pub fn from_message(msg: &paho_mqtt::Message, device_name: &str) -> Option<Instruction> {
        match msg.topic() {
            command_topic if command_topic == device_name => {
                // TODO just convert command state to take from_str() From impl
                let maybe_state = CommandState::try_from(msg.payload()).ok();
                Some(Instruction::Command(maybe_state?))
            },
            command_topic if command_topic == format!("{}/brightness", device_name) => {
                let brightness_str = String::from_utf8(msg.payload().to_vec()).ok()?;
                Some(Instruction::Brightness(brightness_str.parse::<u8>().ok()?))
            },
            command_topic if command_topic == format!("{}/rgb", device_name) => {
                let rgb_str = String::from_utf8(msg.payload().to_vec()).ok()?;
                Some(Instruction::Rgb(Rgb::from_str(&rgb_str).ok()?))
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rgb(u8, u8, u8);

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
        .collect();

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
pub enum RgbFromError {
    #[error("rgb string initializer requires that there are 3 color u8 values separated by commas")]
    InvalidStructure,
    #[error("error while parsing one of the elements of the Rgb string to u8")]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    mod rgb {
        use super::*;

        #[rstest(input, expected,
            case("0,0,0", Rgb(0, 0, 0)),
            case("123,123,123", Rgb(123, 123, 123)),
            case("255,255,255", Rgb(255, 255, 255))
        )]
        fn test_from_str(input: &str, expected: Rgb) {
            assert_eq!(Rgb::from_str(input).unwrap(), expected);
        }

        #[rstest(input,
            case("0,asdf,0"),
            case("asdf,asdf,asdf"),
            case("0,0,0,0"),
            case("asdf,asdf,asdf,asdf"),
            case("0"),
            case("0,0,"),
            case(",,,"),
        )]
        fn test_from_str_rgb_err(input: &str) {
            assert!(Rgb::from_str(input).is_err());
        }

        #[rstest]
        fn test_to_string() {
            assert_eq!(Rgb(0, 0, 0).to_string(), "0,0,0");
        }
    }

    mod instruction {
        use super::*;

        #[rstest(input, expected,
            // COMMAND
            case(paho_mqtt::Message::new(
                "test", "ON", 0
            ), Some(Instruction::Command(CommandState::On))),
            case(paho_mqtt::Message::new(
                "test", "OFF", 0
            ), Some(Instruction::Command(CommandState::Off))),
            case(paho_mqtt::Message::new(
                "test", "invalid message", 0
            ), None),
            // BRIGHTNESS
            case(paho_mqtt::Message::new(
                "test/brightness", "255", 0
            ), Some(Instruction::Brightness(255))),
            case(paho_mqtt::Message::new(
                "test/brightness", [0x32, 0x35, 0x35], 0
            ), Some(Instruction::Brightness(255))),
            case(paho_mqtt::Message::new(
                "test/brightness", [0xc3, 0x28], 0
            ), None),
            case(paho_mqtt::Message::new(
                "test/brightness", "256", 0
            ), None),
            case(paho_mqtt::Message::new(
                "test/brightness", "asdf", 0
            ), None),
            case(paho_mqtt::Message::new(
                "test/brightness", "-1", 0
            ), None),
            // RGB
            case(paho_mqtt::Message::new(
                "test/rgb", "0,0,0", 0
            ), Some(Instruction::Rgb(Rgb(0, 0, 0)))),
            case(paho_mqtt::Message::new(
                "test/rgb", "123,asdf,123", 0
            ), None),
            case(paho_mqtt::Message::new(
                "test/rgb", [0xc3, 0x28], 0
            ), None),
        )]
        fn test_from_message(input: paho_mqtt::Message, expected: Option<Instruction>) {
            assert_eq!(Instruction::from_message(&input, "test"), expected);
        }
    }

    mod light {
        use crate::common::{setup_mqtt, Controller};
        use super::*;
        use serial_test::serial;
        use crate::common::command_state::CommandState;

        #[fixture]
        fn light() -> Light {
            Light::default("asdf".to_string(), setup_mqtt("asdf"))
        }

        #[rstest]
        #[serial]
        fn test_handle_brightness(mut light: Light) {
            light.handle_brightness(255);
            assert_eq!(light.led_controller.pins()["white"].get_desired_brightness(), 255);
            for i in ["red", "green", "blue"] {
                assert_eq!(light.led_controller.pins()[&i.to_string()].get_desired_brightness(), 0);
            }
        }

        #[rstest]
        #[serial]
        fn test_handle_rgb(mut light: Light) {
            light.handle_rgb(&Rgb(255, 255, 255));
            assert_eq!(light.led_controller.pins()["white"].get_desired_brightness(), 0);
            for i in ["red", "green", "blue"] {
                assert_eq!(light.led_controller.pins()[&i.to_string()].get_desired_brightness(), 255);
            }
        }

        #[rstest]
        #[serial]
        fn test_get_rgb_state(mut light: Light) {
            light.handle_rgb(&Rgb(255, 255, 255));
            assert_eq!(light.get_rgb_state(), Rgb(0, 0, 0));
            light.handle_command(&CommandState::On);
            assert_eq!(light.get_rgb_state(), Rgb(255, 255, 255));
        }

        #[rstest(msg, inst, expected,
            case(paho_mqtt::Message::new("asdf", "ON", 0), Instruction::Command(CommandState::On), vec![paho_mqtt::Message::new("asdf/state", "ON", 0)]),
            case(
                paho_mqtt::Message::new("asdf/brightness", "255", 0),
                Instruction::Brightness(255),
                vec![
                    paho_mqtt::Message::new("asdf/brightness/state", "255", 0), 
                    paho_mqtt::Message::new("asdf/rgb/state", "0,0,0", 0)]
                )
        )]
        #[serial]
        fn test_handle_action_and_get_states_to_send(mut light: Light, msg: paho_mqtt::Message, inst: Instruction, expected: Vec<paho_mqtt::Message>) {
            light.handle_action(msg);
            let actual = light.get_states_to_send(inst);
            for (i, msg) in actual.iter().enumerate() {
                let expected_at_i = &expected[i];
                assert_eq!(msg.topic(), expected_at_i.topic());
                assert_eq!(msg.payload(), expected_at_i.payload());
            }
        }
    }
}
