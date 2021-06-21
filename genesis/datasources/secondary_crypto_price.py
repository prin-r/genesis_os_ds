#!/usr/bin/env python3

import sys
import json
import requests

URL = "https://asia-southeast2-price-caching.cloudfunctions.net/query-price"
HEADERS = {"Content-Type": "application/json"}


def main(symbol):
    payload = {"source": "cmc", "symbols": symbol.split("_")}
    [base, quote] = pxs = requests.request("POST", URL, headers=HEADERS, json=payload).json()
    return float(base) / float(quote)


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
