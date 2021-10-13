#include <Utils.h>
#include <Arduino.h>

String Utils::bytePointerToString(byte* pointer, int length) {
    String payloadStr;
    for (int i = 0; i < length; i++) {
        payloadStr += (char)pointer[i];
    }
    return payloadStr;
}

void Utils::writeStringToCharArr(String src, char* dest) {
    int srcLen = src.length() + 1;
    src.toCharArray(dest, srcLen);
}
