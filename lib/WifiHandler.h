#ifndef WifiHandler_h
#define WifiHandler_h

#include <ArduinoOTA.h>

class WifiHandler {
    public:
        WifiHandler();
        WifiHandler(char* ssid, char* password);
        void init();
        WiFiClient net;
    private:
        char* ssid;
        char* password;
};

#endif