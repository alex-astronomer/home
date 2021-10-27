#ifndef MQTTHandler_h
#define MQTTHandler_h

#include <ArduinoOTA.h>
#include <PubSubClient.h>
#include <Light.h>
#include <Logger.h>

class MQTTHandler {
    public:
        MQTTHandler();
        MQTTHandler(char* user, char* pass, Light &light);
        void init(WiFiClient &net);
        PubSubClient client;
        void sendState();
    protected:
        Logger logger;
        std::map<String, std::function<bool(String)>> callbackFnMap;
    private:
        char* user;
        char* pass;
        Light lightController = Light();
        bool handleCommand(String command);
        bool handleBrightnessCommand(String brightness);
        bool handleRgbCommand(String rgbTuple);
        bool handleStartup(String status);
        void callback(char* topic, byte* payload, unsigned int length);
        String deviceName = String("dev");
        String brightnessTopic = deviceName + "/brightness";
        String rgbTopic = deviceName + "/rgb";
        String stateTopic = deviceName;
};

#endif