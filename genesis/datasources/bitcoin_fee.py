#!/usr/bin/env python3

import sys
import json
import urllib.request

BLOCKCHAIN_URL = "https://bitcoinfees.earn.com/api/v1/fees/recommended"


def make_json_request(url):
    req = urllib.request.Request(url)
    req.add_header(
        "User-Agent",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36",
    )
    return urllib.request.urlopen(req).read()


def main():
    raw = make_json_request(BLOCKCHAIN_URL).decode()
    rawJSON = json.loads(raw)
    return f"{rawJSON['fastestFee']} {rawJSON['halfHourFee']} {rawJSON['hourFee']}"


if __name__ == "__main__":
    try:
        print(main())
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
