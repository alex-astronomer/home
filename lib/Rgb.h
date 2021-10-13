#ifndef Rgb_h
#define Rgb_h

#include <Arduino.h>

class Rgb {
    public:
        Rgb(String rgbTuple);
        int red = 0;
        int green = 0;
        int blue = 0;
};

#endif