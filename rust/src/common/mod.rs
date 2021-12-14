pub mod command_state;
use command_state::CommandState;
use super::hardware::{
    pins::Pin, 
    led_controller::LedController
};

pub trait Controller<T: Pin> {
    type InstructionType;

    fn handle_command(&mut self, state: &CommandState) {
        match state {
            CommandState::On => self.get_led_controller().on(),
            CommandState::Off => self.get_led_controller().off()
        };
        self.get_led_controller().set_on_state(state.into());
    }
    
    fn handle_action(&mut self, action: paho_mqtt::Message);
    fn default(name: String, mqtt_client: paho_mqtt::Client) -> Self;
    fn get_states_to_send(&self, inst: Self::InstructionType) -> Vec<paho_mqtt::Message>;
    fn get_led_controller(&mut self) -> &mut LedController<T>;
}

pub fn setup_mqtt(client_id: &str) -> paho_mqtt::Client {
    let client = paho_mqtt::Client::new(
        paho_mqtt::CreateOptionsBuilder::new()
            .server_uri("10.0.0.40:1883")
            .client_id(client_id)
            .finalize()
    ).expect("Error creating MQTT client.");
    client.connect(
        paho_mqtt::ConnectOptionsBuilder::new()
            .user_name("alex")
            .password("assblood")
            .keep_alive_interval(std::time::Duration::from_secs(120))
            .will_message(paho_mqtt::Message::new(format!("{}/available", client_id), "0", 0))
            .finalize()
    ).expect("Error connecting MQTT client.");
    client
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use serial_test::serial;
    use super::command_state::CommandState;
    use crate::devices::light::Light;
    use paho_mqtt::Client;
    use super::*;

    #[fixture]
    fn light() -> Light {
        Light::new("test".to_string(), Client::new(
            paho_mqtt::CreateOptionsBuilder::new()
                .server_uri("10.0.0.40:1883")
                .client_id("test")
                .finalize()
            ).unwrap()
        )
    }

    #[rstest(input, expected,
        case(&CommandState::On, true),
        case(&CommandState::Off, false)
    )]
    #[serial]
    fn test_handle_command(mut light: Light, input: &CommandState, expected: bool) {
        light.handle_command(input);
        assert_eq!(light.get_led_controller().on_state(), expected);
    }

    #[rstest]
    fn test_setup_mqtt() {
        setup_mqtt("test_setup");
    }
}
