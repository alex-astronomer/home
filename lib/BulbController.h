#ifndef BulbController_h
#define BulbController_h

#include <WifiHandler.h>
#include <MQTTHandler.h>
#include <Light.h>
#include <Arduino.h>

class BulbController {
    public:
        BulbController(char* ssid, char* wifiPassword, char* mqttUser, char* mqttPassword);
        void init();
        void loop();
    private:
        WifiHandler wifiHandler;
        MQTTHandler mqttHandler;
        Light light = Light();
        void reconnect();
};

#endif