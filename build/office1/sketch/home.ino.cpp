#include <Arduino.h>
#line 1 "/deploy/home/home.ino"
#include <BulbController.h>
#include <ArduinoOTA.h>

char* ssid = "My Names Not Rick";
char* password = "sp0ngeb0b840!";
char* user = "alex";
char* mqttpass = "assblood";

BulbController bc = BulbController(ssid, password, user, mqttpass);

#line 11 "/deploy/home/home.ino"
void setup();
#line 16 "/deploy/home/home.ino"
void loop();
#line 11 "/deploy/home/home.ino"
void setup() {
  bc.init();
  ArduinoOTA.begin();
}

void loop() {
  ArduinoOTA.handle();
  bc.loop();
}

