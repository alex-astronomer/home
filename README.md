# home
## installation
Clear flash
```bash
pyenv exec esptool.py --port /dev/tty.usbserial-230 erase_flash
```
Upload the binary
```bash
pyenv exec esptool.py --port /dev/cu.usbserial-230 --baud 460800 write_flash --flash_size=detect -fm dout 0 ~/Downloads/esp8266-1m-20210902-v1.17.bin
```
Enable webrepl and set password
```python
import webrepl_setup
```