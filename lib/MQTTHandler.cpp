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

void MQTTHandler::publishStringToState(char* stateTopic, String statePayloadStr) {
    int commandLength = statePayloadStr.length() + 1;
    char returnState[statePayloadStr.length() + 1];
    statePayloadStr.toCharArray(returnState, commandLength);
    client.publish(stateTopic, returnState);
}

void MQTTHandler::handleCommand(String command) {
    char stateTopic[] = "dev/state";
    if (command == "ON") {
        lightController.on();
        publishStringToState(stateTopic, command);
    } else if (command == "OFF") {
        lightController.off();   
        publishStringToState(stateTopic, command);
    }
}

void MQTTHandler::handleBrightnessCommand(String brightness) {
    lightController.setBrightness(brightness.toInt());
    publishStringToState("dev/brightness/state", brightness);
}

void MQTTHandler::handleRgbCommand(String csvRgb) {
    Rgb rgb = Rgb(csvRgb);
    lightController.setRgb(rgb.red, rgb.green, rgb.blue);
    publishStringToState("dev/rgb/state", csvRgb);
}

void MQTTHandler::callback(char* topic, byte* payload, unsigned int length) {
    std::map<String, std::function<void(String)>> callbackFnMap = {
        {"dev", std::bind(&MQTTHandler::handleCommand, this, std::placeholders::_1)},
        {"dev/brightness", std::bind(&MQTTHandler::handleBrightnessCommand, this, std::placeholders::_1)},
        {"dev/rgb", std::bind(&MQTTHandler::handleRgbCommand, this, std::placeholders::_1)}
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
    while (!client.connected()) {
        if (!client.connect("dev", "alex", "assblood", "dev/available", 0, true, "0")) {
            delay(1000);
        }
    }
    client.publish("dev/available", "1", true);
    client.subscribe("dev/#");
}
