#!/usr/bin/env python3

import sys
import json
import requests

URL = "https://asia-southeast2-price-caching.cloudfunctions.net/query-price"
HEADERS = {"Content-Type": "application/json"}

SYMBOLS_WITH_WEIGHTS = [
    ["BTC", 0.2597],
    ["ETH", 0.2208],
    ["XTZ", 0.04545],
    ["BNB", 0.04545],
    ["ATOM", 0.04545],
    ["EOS", 0.04545],
    ["OKB", 0.01429],
    ["CRO", 0.01429],
    ["ALGO", 0.01429],
    ["ADA", 0.01429],
    ["XRP", 0.01429],
    ["BCH", 0.01429],
    ["OMG", 0.01429],
    ["HT", 0.01429],
    ["TRX", 0.01429],
    ["XLM", 0.01429],
    ["ZRX", 0.01039],
    ["KNC", 0.01039],
    ["THETA", 0.01039],
    ["NEO", 0.01039],
    ["LEO", 0.01039],
    ["VET", 0.01039],
    ["LTC", 0.01039],
    ["LINK", 0.01039],
    ["BAT", 0.01039],
    ["MKR", 0.01039],
    # ["QTUM", 0.0649],
    ["DGB", 0.0052],
    ["ONT", 0.0052],
    ["MIOTA", 0.0052],
    ["ICX", 0.0052],
    ["XEM", 0.0052],
]


def main():
    [symbols, weights] = list(zip(*SYMBOLS_WITH_WEIGHTS))
    payload = {"source": "cmc", "symbols": symbols}
    prices = requests.request("POST", URL, headers=HEADERS, json=payload).json()
    acc = 0
    for (p, w) in zip(prices, weights):
        acc += float(p) * w
    return acc


if __name__ == "__main__":
    try:
        print(main())
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
