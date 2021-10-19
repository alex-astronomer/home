#include <BulbController.h>
#include <WifiHandler.h>
#include <MQTTHandler.h>

BulbController::BulbController (char* ssid, char* wifiPassword, char* mqttUser, char* mqttPassword) {
    wifiHandler = WifiHandler(ssid, wifiPassword);
    mqttHandler = MQTTHandler(mqttUser, mqttPassword, light);
}

void BulbController::init() {
    light.init();
    wifiHandler.init();
    mqttHandler.init(wifiHandler.net);
    mqttHandler.publishString(String(SPEC) + "/ip", WiFi.localIP().toString());
    mqttHandler.sendState();
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
