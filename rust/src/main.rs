use paho_mqtt;
mod devices;
mod common;
mod hardware;
use devices::light::Light;
use devices::outlet::Outlet;
use crate::common::Controller;
use common::setup_mqtt;

fn main() {
    let client_id = "rust-dev";

    let mut mqtt_client = setup_mqtt(client_id);
    mqtt_client.subscribe(client_id, 0).expect("Unable to subscribe.");
    mqtt_client.subscribe(&format!("{}/#", client_id), 0).expect("Unable to subscribe.");
    mqtt_client.publish(paho_mqtt::Message::new(format!("{}/available", client_id), "1", 0))
        .expect("message send failure");

    let receiver = mqtt_client.start_consuming();
    let mut light = Outlet::default(String::from(client_id), mqtt_client);
    loop {
        if let Ok(Some(msg)) = receiver.try_recv() {
            light.handle_action(msg);
        }
    }
}
