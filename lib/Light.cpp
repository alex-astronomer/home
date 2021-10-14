#include <Light.h>
#include <Arduino.h>

Light::Light() {}

void Light::init() {
    for (int i = 0; i < sizeof(allPins)/sizeof(allPins[0]); i++) {
        pinMode(allPins[i], OUTPUT);
        analogWrite(allPins[i], pinBrightness[i]);
    }
}

void Light::blink(int pin) {
    int delayMs = 250;
    digitalWrite(pin, LOW);
    delay(delayMs);
    digitalWrite(pin, HIGH);
    delay(delayMs);
    digitalWrite(pin, LOW);
}

void Light::on() {
    for (int i = 0; i < sizeof(allPins)/sizeof(allPins[0]); i++) {
        analogWrite(allPins[i], pinBrightness[i]);
    }
}

void Light::off() {
    for (int i = 0; i < sizeof(allPins)/sizeof(allPins[0]); i++) {
        digitalWrite(allPins[i], LOW);
    }
}

void Light::setBrightness(int brightness) {
    int brightnessArr[4] = {brightness, 0, 0, 0};
    setPinBrightness(brightnessArr);
}

void Light::setRgb(int red, int green, int blue) {
    int brightnessArr[4] = {0, red, green, blue};
    setPinBrightness(brightnessArr);
}

void Light::setPinBrightness(int brightnessArr[4]) {
    for (int i = 0; i < 4; i++) {
        pinBrightness[i] = brightnessArr[i];
    }
}
