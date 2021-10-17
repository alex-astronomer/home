import subprocess
import multiprocessing
import shlex
import yaml
import espota
from datetime import datetime


def compile_and_upload(bulb):
    compile_command = f"""
        arduino-cli compile 
        -b esp8266:esp8266:wifi_slot 
        --build-path /Users/alexkennedy/Developer/home/build/{bulb["name"]}
        --build-cache-path /Users/alexkennedy/Developer/home/build/{bulb["name"]}/cache 
        --build-property "build.extra_flags=-DESP8266 -DSPEC=\\"{bulb["name"]}\\"" 
        --library /Users/alexkennedy/Developer/home/lib 
        /Users/alexkennedy/Developer/home
        """
    subprocess.run(shlex.split(compile_command), universal_newlines=True)
    espota.main([
        '/Users/alexkennedy/Developer/home/deploy/espota.py',
        '-i',
        bulb["ip"],
        '-f',
        f'/Users/alexkennedy/Developer/home/build/{bulb["name"]}/home.ino.bin'
    ])


with open("./config.yaml", "r") as f:
    bulb_config = yaml.load(f)

processes = []

for bulb in bulb_config["prod"]:
    p = multiprocessing.Process(
        target=compile_and_upload,
        args=(bulb,),
    )
    p.start()
    processes.append(p)

for p in processes:
    p.join()
