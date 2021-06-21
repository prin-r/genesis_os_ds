#!/usr/bin/env python3

import sys
import json
import requests

HEADERS = {"Content-Type": "application/json"}


def main(url, symbols):
    payload = {"symbols": symbols, "min_count": 10, "ask_count": 16}
    result = requests.request("POST", url, headers=HEADERS, json=payload).json()
    return " ".join([str(float(px["px"]) / float(px["multiplier"])) for px in result["result"]])


if __name__ == "__main__":
    try:
        print(main(sys.argv[1],[*sys.argv[2:]]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
