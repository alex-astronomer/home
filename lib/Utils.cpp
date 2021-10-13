#include <Utils.h>
#include <Arduino.h>

String Utils::bytePointerToString(byte* pointer, int length) {
    String payloadStr;
    for (int i = 0; i < length; i++) {
        payloadStr += (char)pointer[i];
    }
    return payloadStr;
}
