#!/usr/bin/env python3

import json
import urllib.request
import sys
from datetime import datetime
from operator import itemgetter

CIRIUM_URL = "https://api.flightstats.com/flex/flightstatus/rest/v2/json/flight/status/{}/dep/{}?appId=483244b1&appKey=078d253494c896ea92f2c6e37331c6dc&utc=true"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(flight_number, date):
    res = make_json_request(CIRIUM_URL.format(flight_number, date))
    if "flightStatuses" not in res:
        raise ValueError("key flightStatuses not found")
    if len(res["flightStatuses"]) == 0:
        raise ValueError("flight statuses is empty")

    status, delays, operational_times = itemgetter("status", "delays", "operationalTimes")(
        res["flightStatuses"][0]
    )

    if status == "C":
        # flight cancelled
        return f"{status},{-1}"
    elif status == "D":
        # flight diverted
        return f"{status},{-1}"
    elif status != "L" and status != "A" and status != "C" and status != "D":
        # Unprocessable status
        return f"{status},{-1}"
    else:
        arrived = "actualGateArrival" in operational_times

        if status == "A" or (status == "L" and not arrived):
            # flight still active or not at gate
            return f"{status},{-1}"
        elif status == "L" and arrived:
            sga, aga = [
                int(datetime.strptime(t["dateUtc"], "%Y-%m-%dT%H:%M:%S.%fZ").timestamp())
                for t in itemgetter("scheduledGateArrival", "actualGateArrival")(operational_times)
            ]

            delay_in_minutes = aga - sga
            if delay_in_minutes < 0:
                delay_in_minutes = 0

            return f"{status},{delay_in_minutes // 60}"
        else:
            # no delay info
            return f"{status},{-1}"


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
