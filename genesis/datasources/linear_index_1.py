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
    ["ALGO", 0.020414286],
    ["ADA", 0.020414286],
    ["XRP", 0.020414286],
    ["BCH", 0.020414286],
    ["HT", 0.020414286],
    ["TRX", 0.020414286],
    ["XLM", 0.020414286],
    ["ZRX", 0.017316667],
    ["NEO", 0.017316667],
    ["VET", 0.017316667],
    ["LTC", 0.017316667],
    ["LINK", 0.017316667],
    ["BAT", 0.017316667],
    # ["QTUM", 0.0649]
    ["ONT", 0.013],
    ["XEM", 0.013],
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
