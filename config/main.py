import yaml


if __name__ == '__main__':
    with open("./bulbTemplate.yml", "r") as f:
        template_string = f.read()

    light_config = {
        "entities": ["dev", "office1", "office2", "dining1", "dining2", "living1", "living2"],
        "groups": {
            "Dining Room Lights": ["dining1", "dining2"],
            "Living Room Lights": ["living1", "living2"],
            "Office Lights": ["office1", "office2"],
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
