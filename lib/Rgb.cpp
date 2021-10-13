#include <Rgb.h>
#include <Arduino.h>

Rgb::Rgb(String rgbTuple) {
    int start = 0;
    int end = rgbTuple.length();
    int rgbArr[3];
    for (int i = 0; i < 3; i++) {
        String currentSubStr = rgbTuple.substring(start, end);
        int commaIndex = currentSubStr.indexOf(',');
        if (commaIndex > 0) {
            rgbArr[i] = currentSubStr.substring(0, commaIndex).toInt();
        } else {
            rgbArr[i] = currentSubStr.toInt();
        }
        start = start + commaIndex + 1;
    }
    red = rgbArr[0];
    green = rgbArr[1];
    blue = rgbArr[2];
}
