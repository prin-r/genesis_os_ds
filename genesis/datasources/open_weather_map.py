#!/usr/bin/env python3

import json
import urllib.request
import sys

OPEN_WEATHER_MAP_URL = "https://api.openweathermap.org/data/2.5/weather?q={}&appid=ac7c05361f8f91652eab609377134ab7"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(city, main_field, sub_field):
    random = make_json_request(OPEN_WEATHER_MAP_URL.format(city))
    return str(random[main_field][sub_field])
    raise ValueError("unknown city, main_field, or sub_field")


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
