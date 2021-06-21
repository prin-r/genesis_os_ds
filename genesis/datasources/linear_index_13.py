#!/usr/bin/env python3

import sys
import json
import requests

URL = "https://asia-southeast2-price-caching.cloudfunctions.net/query-price"
HEADERS = {"Content-Type": "application/json"}

SYMBOLS_WITH_WEIGHTS = [
    ["BTC", 0.2597],
    ["ETH", 0.2208],
    ["XTZ", 0.03636],
    ["BNB", 0.03636],
    ["ATOM", 0.03636],
    ["YFI", 0.03636],
    ["EOS", 0.03636],
    ["OKB", 0.011908333],
    ["CRO", 0.011908333],
    ["ALGO", 0.011908333],
    ["ADA", 0.011908333],
    ["SNX", 0.011908333],
    ["COMP", 0.011908333],
    ["BCH", 0.011908333],
    ["OMG", 0.011908333],
    ["DOT", 0.011908333],
    ["HT", 0.011908333],
    ["TRX", 0.011908333],
    ["XLM", 0.011908333],
    ["XRP", 0.009445455],
    ["ZRX", 0.009445455],
    ["KNC", 0.009445455],
    ["THETA", 0.009445455],
    ["NEO", 0.009445455],
    ["VET", 0.009445455],
    ["LTC", 0.009445455],
    ["LINK", 0.009445455],
    ["AAVE", 0.009445455],
    ["BAT", 0.009445455],
    ["MKR", 0.009445455],
    # ["QTUM", 0.021633333],
    ["BTT", 0.021633333],
    ["REN", 0.021633333],
    ["DGB", 0.003714286],
    ["KSM", 0.003714286],
    ["ONT", 0.003714286],
    ["MIOTA", 0.003714286],
    ["ICX", 0.003714286],
    ["XEM", 0.003714286],
    # ["UMA", 0.003714286],
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
