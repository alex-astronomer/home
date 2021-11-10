mod light;

pub struct Light {
    name: String,
    mqtt_client: paho_mqtt::Client,
}

impl Light {
    pub fn new(name: String, mqtt_client: paho_mqtt::Client) -> Light {
        Light {name, mqtt_client}
    }

    pub fn handle_action(&self, action: Instruction) {
        // run command
        match action {
            Instruction::Command(ref state) => self.command(&state),
            Instruction::Brightness(brightness) => self.brightness(brightness),
            Instruction::Rgb(ref rgb) => self.rgb(rgb),
        };
    }

    fn command(&self, state: &CommandState) {
        println!("Light turned {} placeholder.", state.to_str());
        self.mqtt_client.publish(
            paho_mqtt::Message::new(
                format!("{}/state", self.name), state.to_str(), 0
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
                format!("{}/state", self.name), rgb.to_str(), 0
            )
        ).expect(
            "Rgb state mismatch! Command state change failed to send. God speed, sir. *salutes*"
        );
    }
}

pub enum Instruction {
    Command(CommandState),
    Brightness(u8),
    Rgb(Rgb),
}

impl Instruction {
    pub fn from_message(msg: paho_mqtt::Message, for_device: &Light) -> Option<Instruction> {
        match msg.topic() {
            command_topic if command_topic == &for_device.name => {
                let state_option = CommandState::from_bytes(msg.payload());
                match state_option {
                    Some(state) => Some(Instruction::Command(state)),
                    None => None,
                }
            },
            command_topic if command_topic == format!("{}/brightness", for_device.name) => {
                let brightness = String::from_utf8(msg.payload().to_vec())
                    .unwrap()
                    .parse::<u8>();
                match brightness {
                    Ok(brightness) => Some(Instruction::Brightness(brightness)),
                    _ => None
                }
            },
            command_topic if command_topic == format!("{}/rgb", for_device.name) => {
                let rgb_str = String::from_utf8(msg.payload().to_vec());
                Some(Instruction::Rgb(Rgb::from_str(&rgb_str.unwrap()[..])))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum CommandState {
    On,
    Off,
}

impl CommandState {
    fn from_bytes(bytes: &[u8]) -> Option<CommandState> {
        match &String::from_utf8(bytes.to_vec()).unwrap()[..] {
            "ON" => Some(CommandState::On),
            "OFF" => Some(CommandState::Off),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            CommandState::On => "ON",
            CommandState::Off => "OFF",
        }
    }
}

#[derive(Debug)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    fn from_str(from: &str) -> Rgb {
        let vec: Vec<u8> = from.split(',').map(|n| n.parse::<u8>().unwrap()).collect();
        Rgb(vec[0], vec[1], vec[2])
    }

    fn to_str(&self) -> String {
        format!("{},{},{}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    mod rgb {
        #[test]
        fn test_from_str() {
            let csv = "100,100,100";
            let rgb = light::Rgb::from_str(csv);
        }
    }
}
