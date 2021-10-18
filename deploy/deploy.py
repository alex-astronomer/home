from os import path
import subprocess
import multiprocessing
import shlex
import yaml
import espota


def compile_and_upload(bulb):
    if not path.exists(f'/deploy/build/{bulb["name"]}/home.ino.bin'):
        compile_command = f"""
            arduino-cli compile 
            -b esp8266:esp8266:wifi_slot 
            --build-path /deploy/build/{bulb["name"]}
            --build-cache-path /deploy/build/{bulb["name"]}/cache 
            --build-property "build.extra_flags=-DESP8266 -DSPEC=\\"{bulb["name"]}\\"" 
            --library /deploy/lib
            /deploy/home/home.ino
            """
        subprocess.run(shlex.split(compile_command), universal_newlines=True)
    espota.main([
        '/deploy/scripts/espota.py',
        '-i',
        bulb["ip"],
        '-f',
        f'/deploy/build/{bulb["name"]}/home.ino.bin'
    ])


with open("./config.yaml", "r") as f:
    bulb_config = yaml.load(f, Loader=yaml.Loader)

processes = []

for bulb in bulb_config["stage"]:
    p = multiprocessing.Process(
        target=compile_and_upload,
        args=(bulb,),
    )
    p.start()
    processes.append(p)

for p in processes:
    p.join()
