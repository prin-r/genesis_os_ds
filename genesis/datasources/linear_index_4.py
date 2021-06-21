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
    ["OKB", 0.015877778],
    ["CRO", 0.015877778],
    ["ALGO", 0.015877778],
    ["ADA", 0.015877778],
    ["XRP", 0.015877778],
    ["BCH", 0.015877778],
    ["HT", 0.015877778],
    ["TRX", 0.015877778],
    ["XLM", 0.015877778],
    ["ZRX", 0.014842857],
    ["KNC", 0.014842857],
    ["NEO", 0.014842857],
    ["VET", 0.014842857],
    ["LTC", 0.014842857],
    ["LINK", 0.014842857],
    ["BAT", 0.014842857],
    # ["QTUM", 0.0649],
    ["ONT", 0.0065],
    ["MIOTA", 0.0065],
    ["ICX", 0.0065],
    ["XEM", 0.0065],
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
