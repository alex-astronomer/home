#include <MQTTHandler.h>
#include <ArduinoOTA.h>
#include <Utils.h>
#include <map>
#include <Rgb.h>

MQTTHandler::MQTTHandler() {}

MQTTHandler::MQTTHandler(char* user, char* pass, Light &light) {
    this->user = user;
    this->pass = pass;
    this->lightController = light;
}

void MQTTHandler::publishString(String topic, String payload) {
    int payloadLen = payload.length() + 1;
    char payloadArr[payloadLen];
    payload.toCharArray(payloadArr, payloadLen);

    int topicLength = topic.length() + 1;
    char topicArr[topicLength];
    topic.toCharArray(topicArr, topicLength);
    
    client.publish(topicArr, payloadArr);
}

void MQTTHandler::handleCommand(String command) {
    // lightController.blink(4);
    String stateTopic = deviceName + "/state";
    if (command == "ON") {
        lightController.on();
        publishString(stateTopic, command);
    } else if (command == "OFF") {
        lightController.off();   
        publishString(stateTopic, command);
    }
}

void MQTTHandler::handleBrightnessCommand(String brightness) {
    lightController.setBrightness(brightness.toInt());
    publishString(deviceName + "/brightness/state", brightness);
}

void MQTTHandler::handleRgbCommand(String csvRgb) {
    Rgb rgb = Rgb(csvRgb);
    lightController.setRgb(rgb.red, rgb.green, rgb.blue);
    publishString(deviceName + "/rgb/state", csvRgb);
}

void MQTTHandler::callback(char* topic, byte* payload, unsigned int length) {
    // lightController.blink(14);
    std::map<String, std::function<void(String)>> callbackFnMap = {
        {deviceName, std::bind(&MQTTHandler::handleCommand, this, std::placeholders::_1)},
        {deviceName + "/brightness", std::bind(&MQTTHandler::handleBrightnessCommand, this, std::placeholders::_1)},
        {deviceName + "/rgb", std::bind(&MQTTHandler::handleRgbCommand, this, std::placeholders::_1)}
    };
    String payloadStr = Utils::bytePointerToString(payload, length);
    if (callbackFnMap.find(topic) != callbackFnMap.end()) {
        callbackFnMap[topic](payloadStr);
    }
}

void MQTTHandler::init(WiFiClient &net) {
    client.setClient(net);
    client.setServer("10.0.0.40", 1883);
    client.setCallback(
        std::bind(&MQTTHandler::callback, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3)
    );
    String availableStateStr = deviceName + "/available";
    int availableStateLen = availableStateStr.length() + 1;
    char stateArr[availableStateLen];
    availableStateStr.toCharArray(stateArr, availableStateLen);
    while (!client.connected()) {
        if (!client.connect(SPEC, "alex", "assblood", stateArr, 0, true, "0")) {
            delay(1000);
        }
    }
    client.publish(stateArr, "1", true);

    availableStateStr = deviceName + "/#";
    availableStateLen = availableStateStr.length() + 1;
    char subscribeArr[availableStateLen];
    availableStateStr.toCharArray(subscribeArr, availableStateLen);
    client.subscribe(subscribeArr);
}
