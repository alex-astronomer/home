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
    private:
        char* user;
        char* pass;
        Light lightController = Light();
        void publishString(String topic, String payload);
        void handleCommand(String command);
        void handleBrightnessCommand(String brightness);
        void handleRgbCommand(String rgbTuple);
        void callback(char* topic, byte* payload, unsigned int length);
        String deviceName = String(SPEC);
};

#endif