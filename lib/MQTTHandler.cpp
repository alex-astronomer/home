#include <MQTTHandler.h>
#include <ArduinoOTA.h>
#include <Utils.h>
#include <map>
#include <tuple>
#include <Rgb.h>

MQTTHandler::MQTTHandler() {}

MQTTHandler::MQTTHandler(char* user, char* pass, Light &light) {
    this->user = user;
    this->pass = pass;
    this->lightController = light;
}

void MQTTHandler::publishString(String topic, String payload) {
    char topicArr[topic.length() + 1];
    Utils::writeStringToCharArr(topic, topicArr);

    char payloadArr[payload.length() + 1];
    Utils::writeStringToCharArr(payload, payloadArr);
    
    client.publish(topicArr, payloadArr, true);
}

bool MQTTHandler::handleCommand(String command) {
    String stateTopic = deviceName + "/state";
    if (command == "ON") {
        lightController.on();
        return true;
    } else if (command == "OFF") {
        lightController.off();   
        return true;
    }
    return false;
}

bool MQTTHandler::handleBrightnessCommand(String brightness) {
    lightController.setBrightness(brightness.toInt());
    return true;
}

bool MQTTHandler::handleRgbCommand(String csvRgb) {
    Rgb rgb = Rgb(csvRgb);
    lightController.setRgb(rgb.red, rgb.green, rgb.blue);
    return true;
}

bool MQTTHandler::handleStartup(String status) {
    if (status == "online") {
        return true;
    }
    return false;
}

void MQTTHandler::sendState() {
    std::map<String, String> state = lightController.getState();
    std::tuple<String, String> updates[3] = {
        {brightnessTopic, state["brightness"]},
        {rgbTopic, state["rgb"]},
        {stateTopic, state["state"]}
    };
    for (int i = 0; i < sizeof(updates)/sizeof(updates[0]); i++) {
        std::tuple<String, String> currentUpdate = updates[i];
        publishString(std::get<0>(currentUpdate) + "/state", std::get<1>(currentUpdate));
    }
}

void MQTTHandler::callback(char* topic, byte* payload, unsigned int length) {
    std::map<String, std::function<bool(String)>> callbackFnMap = {
        {stateTopic, std::bind(&MQTTHandler::handleCommand, this, std::placeholders::_1)},
        {brightnessTopic, std::bind(&MQTTHandler::handleBrightnessCommand, this, std::placeholders::_1)},
        {rgbTopic, std::bind(&MQTTHandler::handleRgbCommand, this, std::placeholders::_1)},
        {"homeassistant/status", std::bind(&MQTTHandler::handleStartup, this, std::placeholders::_1)}
    };
    String payloadStr = Utils::bytePointerToString(payload, length);
    if (callbackFnMap.find(topic) != callbackFnMap.end()) {
        bool commandExecuted = callbackFnMap[topic](payloadStr);
        if (commandExecuted) {
            sendState();
        }
    }
}

void MQTTHandler::init(WiFiClient &net) {
    client.setClient(net);
    client.setServer("10.0.0.40", 1883);
    client.setCallback(
        std::bind(&MQTTHandler::callback, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3)
    );
    String availableStateStr = deviceName + "/available";
    char stateArr[availableStateStr.length() + 1];
    Utils::writeStringToCharArr(availableStateStr, stateArr);
    while (!client.connected()) {
        if (!client.connect(SPEC, "alex", "assblood", stateArr, 0, true, "0")) {
            delay(1000);
        }
    }
    client.publish(stateArr, "1", true);

    String subscribeStr = deviceName + "/#";
    char subscribeArr[subscribeStr.length() + 1];
    Utils::writeStringToCharArr(subscribeStr, subscribeArr);
    client.subscribe(subscribeArr);
    client.subscribe("homeassistant/status");
}
