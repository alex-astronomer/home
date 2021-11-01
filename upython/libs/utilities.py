def normalize(brightness, old_max=255, new_max=1023):
    """
    Utility function to normalize

    :param brightness: Brightness value
    :type brightness: Union[str, int]
    :param old_max: Previous value for max, "converting from"
    :type old_max: int
    :param new_max: New value for max, "converting to"
    :type new_max: int
    :return: Normalized brightness
    :rtype: int

    """
    return round(int(brightness) * (new_max / old_max))
