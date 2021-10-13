#ifndef Utils_h
#define Utils_h

#include <Arduino.h>

class Utils {
    public:
        static String bytePointerToString(byte* pointer, int length);
        static void writeStringToCharArr(String src, char* dest);
};

#endif