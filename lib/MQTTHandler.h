#ifndef MQTTHandler_h
#define MQTTHandler_h

#include <ArduinoOTA.h>
#include <PubSubClient.h>
#include <Light.h>

class MQTTHandler {
    public:
        MQTTHandler();
        MQTTHandler(char* user, char* pass, Light &light);
        void init(WiFiClient &net);
        PubSubClient client;
        void publishString(String topic, String payload);
        void sendState();
    private:
        char* user;
        char* pass;
        Light lightController = Light();
        bool handleCommand(String command);
        bool handleBrightnessCommand(String brightness);
        bool handleRgbCommand(String rgbTuple);
        bool handleStartup(String status);
        void callback(char* topic, byte* payload, unsigned int length);
        String deviceName = String(SPEC);
        String brightnessTopic = deviceName + "/brightness";
        String rgbTopic = deviceName + "/rgb";
        String stateTopic = deviceName;
};

#endif