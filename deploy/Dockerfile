FROM python:latest

RUN pip install pyyaml docopt

RUN mkdir -p /arduino-cli/bin
RUN curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | BINDIR=/arduino-cli/bin sh

ENV PATH="/arduino-cli/bin:${PATH}"
RUN arduino-cli core install --additional-urls https://arduino.esp8266.com/stable/package_esp8266com_index.json esp8266:esp8266
