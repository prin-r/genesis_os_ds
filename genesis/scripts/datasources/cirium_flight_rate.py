#!/usr/bin/env python3

import json
import urllib.request
import sys

CIRIUM_URL = "https://api.flightstats.com/flex/ratings/rest/v1/json/flight/{}/{}?appId=483244b1&appKey=078d253494c896ea92f2c6e37331c6dc&departureAirport={}"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(fs_code, flight_number, departure_airport):
    res = make_json_request(CIRIUM_URL.format(
        fs_code, flight_number, departure_airport))
    if "ratings" not in res:
        raise ValueError("key ratings not found")
    if "ontimePercent" not in res["ratings"][0]:
        raise ValueError("key ontimePercent not found")

    return res["ratings"][0]["ontimePercent"]


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
