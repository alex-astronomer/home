use crate::common::{Controller, command_state::CommandState};
use crate::hardware::{pins::DigitalPin, led_controller::LedController};

pub struct Outlet {
    name: String,
    led_controller: LedController<DigitalPin>,
    mqtt_client: paho_mqtt::Client
}

impl Controller<DigitalPin> for Outlet {
    type InstructionType = Instruction;

    fn handle_action(&mut self, action: paho_mqtt::Message) {
        if let Some(inst) = Instruction::from_msg(action, &self.name) {
            match &inst {
                Instruction::Command(state) => { self.handle_command(&state); }
            }
            
            for msg in self.get_states_to_send(inst) {
                self.mqtt_client.publish(msg).expect("mismatched state. see ya!");
            }
        }
    }

    fn default(name: String, mqtt_client: paho_mqtt::Client) -> Self {
        let mut led_controller = LedController::new();
        led_controller.add_pins(vec![
            ("red".to_string(), 22),
            ("blue".to_string(), 23),
            ("relay".to_string(), 27),
        ]);
        Self { name, mqtt_client, led_controller }
    }

    fn get_states_to_send(&self, _: Self::InstructionType) -> Vec<paho_mqtt::Message> {
        vec!(paho_mqtt::Message::new(
            format!("{}/state", self.name),
            CommandState::from(self.led_controller.on_state()).to_string(),
            0
        ))
    }

    fn get_led_controller(&mut self) -> &mut LedController<DigitalPin> {
        &mut self.led_controller
    }
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Command(CommandState),
}

impl Instruction {
    pub fn from_msg(from: paho_mqtt::Message, device_name: &str) -> Option<Self> {
        match from.topic() {
            command_topic if command_topic == device_name => {
                Some(Self::Command(CommandState::try_from(from.payload()).ok()?))
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    mod instruction {
        use super::*;

        #[rstest(topic, payload, expected,
            case("tests_outlet_instruction", "ON", Some(Instruction::Command(CommandState::On))),
            case("tests_outlet_instruction", "OFF", Some(Instruction::Command(CommandState::Off))),
            case("tests_outlet_instruction", "asdf", None),
            case("asdf", "ON", None),
        )]
        fn test_from_msg(topic: String, payload: String, expected: Option<Instruction>) {
            assert_eq!(
                Instruction::from_msg(
                    paho_mqtt::Message::new(topic, payload, 0), 
                    "tests_outlet_instruction"
                ), 
                expected
            );
        }
    }
}
