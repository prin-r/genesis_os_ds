#!/usr/bin/env python3

import sys
import json
import urllib.request

BLOCKCHAIN_URL = "https://bitcoinfees.earn.com/api/v1/fees/recommended"


def make_json_request(url):
    req = urllib.request.Request(url)
    req.add_header("User-Agent", "")
    return urllib.request.urlopen(req).read()


def main(symbol):
    raw = make_json_request(BLOCKCHAIN_URL).decode()
    rawJSON = json.loads(raw)

    [_base, speed] = symbol.split("_")
    if speed == "F":
        return rawJSON["fastestFee"]
    elif speed == "HH":
        return rawJSON["halfHourFee"]
    elif speed == "H":
        return rawJSON["hourFee"]

    raise Exception(f"Error: The symbol {symbol} was not found or misformed.")


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
