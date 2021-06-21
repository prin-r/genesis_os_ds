#!/usr/bin/env python3

import sys
import json
import requests

URL = "https://asia-southeast2-price-caching.cloudfunctions.net/query-price"


def main(symbols):
    results = requests.post(URL, json={"source": "xangle", "symbols": symbols}).json()
    return ",".join(results)


if __name__ == "__main__":
    try:
        print(main(sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
