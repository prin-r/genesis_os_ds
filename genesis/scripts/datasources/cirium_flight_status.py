#!/usr/bin/env python3

import json
import urllib.request
import sys

CIRIUM_URL = "https://api.flightstats.com/flex/flightstatus/rest/v2/json/flight/status/{}/{}/dep/{}?appId=483244b1&appKey=078d253494c896ea92f2c6e37331c6dc&utc=true&airport={}"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(fs_code, flight_number, date, airport):
    res = make_json_request(CIRIUM_URL.format(
        fs_code, flight_number, date, airport))
    if "flightStatuses" not in res:
        raise ValueError("key flightStatuses not found")
    delays = res["flightStatuses"][0]["delays"]
    departure_delay = 0
    arrival_delay = 0
    if "departureGateDelayMinutes" in delays:
        departure_delay = delays["departureGateDelayMinutes"]
    if "arrivalGateDelayMinutes" in delays:
        arrival_delay = delays["arrivalGateDelayMinutes"]
    return f"""{departure_delay} {arrival_delay}"""


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
