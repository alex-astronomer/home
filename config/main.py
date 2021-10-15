import yaml


if __name__ == '__main__':
    with open("./bulbTemplate.yml", "r") as f:
        template_string = f.read()

    light_config = {
        "entities": ["dev", "bulb0", "bulb1", "bulb2", "bulb3", "bulb4", "bulb5"],
        "groups": {
            "Dining Room Lights": ["bulb0", "bulb1"],
            "Living Room Lights": ["bulb2", "bulb3"],
            "Office Lights": ["bulb4", "bulb5"],
        }
    }
    yaml_config = []
    for entity in light_config['entities']:
        yaml_config.append(yaml.load(template_string.replace("bulbName", entity)))
    for group in light_config['groups']:
        yaml_config.append({
            'platform': 'group',
            'name': group,
            'entities': ["light." + bulb for bulb in light_config['groups'][group]]
        })
    with open("./configuration.yaml", "w") as f:
        yaml.dump({"config": yaml_config}, f)
    debug = True
