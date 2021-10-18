# 1 "/deploy/home/home.ino"
# 2 "/deploy/home/home.ino" 2


char* ssid = "My Names Not Rick";
char* password = "sp0ngeb0b840!";
char* user = "alex";
char* mqttpass = "assblood";

BulbController bc = BulbController(ssid, password, user, mqttpass);

void setup() {
  bc.init();
  ArduinoOTA.begin();
}

void loop() {
  ArduinoOTA.handle();
  bc.loop();
}
