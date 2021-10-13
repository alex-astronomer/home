#ifndef Light_h
#define Light_h

class Light {
    public:
        Light();
        void init();
        void on();
        void off();
        void setBrightness(int brightness);
        void setRgb(int red, int green, int blue);
        void blink(int i);
    private:
        int allPins[4] = {5, 4, 12, 14};
        int pinBrightness[4] = {0, 0, 0, 0};
        void setPinBrightness(int brightnessArr[4]);
};

#endif