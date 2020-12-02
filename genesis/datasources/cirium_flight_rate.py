#!/usr/bin/env python3

import json
import urllib.request
import sys
from operator import itemgetter

CIRIUM_URL = "https://api.flightstats.com/flex/ratings/rest/v1/json/flight/{}?appId=483244b1&appKey=078d253494c896ea92f2c6e37331c6dc"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(flight_number):
    res = make_json_request(CIRIUM_URL.format(flight_number))
    if "ratings" not in res:
        raise ValueError("key ratings not found")
    if len(res["ratings"]) == 0:
        raise ValueError("rating is empty")

    return ",".join(
        [
            str(x)
            for x in itemgetter(
                "observations", "late15", "late30", "late45", "cancelled", "diverted"
            )(res["ratings"][0])
        ]
    )


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
