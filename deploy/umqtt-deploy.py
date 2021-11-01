import webrepl_cli
import os
import yaml


def replace_file_inplace(replace_this, with_this):
    main_dir = "/Users/alexkennedy/Developer/home/upython"
    file = "main.py"
    full_path = f"{main_dir}/{file}"
    with open(full_path, "rt") as f:
        data = f.read().replace(replace_this, with_this)
    with open(full_path, "wt") as f:
        f.write(data)


def get_py_files_in_dir(dir):
    files_and_dirs = os.listdir(dir)
    if all([".py" in file for file in files_and_dirs]):
        return [f"{dir}/{file}" for file in files_and_dirs]
    else:
        files = [f"{dir}/{file}" for file in files_and_dirs if ".py" in file]
        for subdir in [subdir for subdir in files_and_dirs if ".py" not in subdir]:
            files = files + [file for file in get_py_files_in_dir(f"{dir}/{subdir}")]

    return files


if __name__ == '__main__':
    with open("/Users/alexkennedy/Developer/home/deploy/config.yaml", "r") as f:
        config = yaml.load(f, Loader=yaml.Loader)
    for bulb_config in config["dev"]:
        device_name = bulb_config["name"]
        main_dir = "/Users/alexkennedy/Developer/home/upython"
        ip = bulb_config["ip"]
        files = get_py_files_in_dir(main_dir)
        dest_files = []
        for file in files:
            split_path = file.split('/')
            upython_i = split_path.index('upython')
            dest_files.append(f'/{"/".join(split_path[upython_i + 1:])}')
        zipper = zip(files, dest_files)

        for src_dst in zipper:
            dest_with_host = f"{ip}:{src_dst[1]}"
            if "main.py" in src_dst[0]:
                replace_file_inplace("!name", f"'{device_name}'")
                webrepl_cli.main(*[
                    "webrepl_cli.py",
                    "-p",
                    "assblood",
                    src_dst[0],
                    dest_with_host,
                ])
                replace_file_inplace(f"'{device_name}'", "!name")
            else:
                webrepl_cli.main(*[
                    "webrepl_cli.py",
                    "-p",
                    "assblood",
                    src_dst[0],
                    dest_with_host,
                ])
