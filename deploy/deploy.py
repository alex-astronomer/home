"""
Deploy command, for Docker.

Usage:
    deploy <env> [--compile | --upload]

Options:
    -h --help       Show this screen.
    -c --compile    Only compile [default: True].
    -u --upload     Only upload [default: True].
"""
import subprocess
import multiprocessing
import shlex
import yaml
import espota
from docopt import docopt


def run_deploy(env, compile, upload):
    def compile_and_upload(bulb, compile, upload):
        print(f"Starting compile for {bulb['name']}.")
        if compile:
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
        if upload:
            espota.main([
                '/deploy/scripts/espota.py',
                '-i',
                bulb["ip"],
                '-f',
                f'/deploy/build/{bulb["name"]}/home.ino.bin'
            ])


    with open("/deploy/scripts/config.yaml", "r") as f:
        bulb_config = yaml.load(f, Loader=yaml.Loader)

    processes = []

    for bulb in bulb_config[env]:
        p = multiprocessing.Process(
            target=compile_and_upload,
            args=(bulb, compile, upload),
        )
        p.start()
        processes.append(p)

    for p in processes:
        p.join()


if __name__ == "__main__":
    args = docopt(__doc__)
    if not args["--compile"] and not args['--upload']:
        args["--compile"], args["--upload"] = True, True
    run_deploy(args["<env>"], args["--compile"], args["--upload"])
