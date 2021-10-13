#include <WifiHandler.h>
#include <ArduinoOTA.h>

WifiHandler::WifiHandler() {}

WifiHandler::WifiHandler(char* ssid, char* password) {
    this->ssid = ssid;
    this->password = password;
}

void WifiHandler::init() {
    WiFi.mode(WIFI_STA);
    WiFi.hostname("dev");
    WiFi.begin(ssid, password);
    while (WiFi.waitForConnectResult() != WL_CONNECTED) {
        delay(5000);
    }
}
