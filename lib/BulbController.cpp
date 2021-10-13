#include <BulbController.h>
#include <WifiHandler.h>
#include <MqttHandler.h>

BulbController::BulbController (char* ssid, char* wifiPassword, char* mqttUser, char* mqttPassword) {
    wifiHandler = WifiHandler(ssid, wifiPassword);
    mqttHandler = MQTTHandler(mqttUser, mqttPassword, light);
}

void BulbController::init() {
    light.init();
    wifiHandler.init();
    mqttHandler.init(wifiHandler.net);
}

void BulbController::loop() {
    reconnect();
    mqttHandler.client.loop();
}

void BulbController::reconnect() {
  if (WiFi.status() != WL_CONNECTED || !mqttHandler.client.connected()) {
    wifiHandler.init();
    mqttHandler.init(wifiHandler.net);
  }
}
