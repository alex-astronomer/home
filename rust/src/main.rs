use paho_mqtt;
mod light;
use light::{Light, Instruction};


fn setup_mqtt(client_id: &str) -> paho_mqtt::Client {
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
            .keep_alive_interval(std::time::Duration::from_secs(5))
            .will_message(paho_mqtt::Message::new(format!("{}/available", client_id), "0", 0))
            .finalize()
    ).expect("Error connecting MQTT client.");
    client.subscribe(client_id, 0).expect("Unable to subscribe.");
    client.subscribe(&format!("{}/#", client_id), 0).expect("Unable to subscribe.");
    client.publish(paho_mqtt::Message::new(format!("{}/available", client_id), "1", 0))
        .expect("message send failure");
    client
}

fn main() {
    let client_id = "rust-dev";
    let mut mqtt_client = setup_mqtt(client_id);
    let receiver = mqtt_client.start_consuming();
    let light = Light::new(String::from(client_id), mqtt_client);

    loop {
        // loop
        if let Ok(Some(msg)) = receiver.try_recv() {
            if let Some(inst) = Instruction::from_message(msg, &light) {
                light.handle_action(inst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
