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
    ["OKB", 0.010992308],
    ["CRO", 0.010992308],
    ["ALGO", 0.010992308],
    ["ADA", 0.010992308],
    ["SNX", 0.010992308],
    ["COMP", 0.010992308],
    ["XRP", 0.010992308],
    ["BCH", 0.010992308],
    ["OMG", 0.010992308],
    ["DOT", 0.010992308],
    ["HT", 0.010992308],
    ["TRX", 0.010992308],
    ["XLM", 0.010992308],
    ["ZRX", 0.01039],
    ["KNC", 0.01039],
    ["THETA", 0.01039],
    ["NEO", 0.01039],
    ["VET", 0.01039],
    ["LTC", 0.01039],
    ["LINK", 0.01039],
    ["AAVE", 0.01039],
    ["BAT", 0.01039],
    ["MKR", 0.01039],
    # ["QTUM", 0.021633333],
    ["BTT", 0.021633333],
    ["REN", 0.021633333],
    ["KSM", 0.004333333],
    ["ONT", 0.004333333],
    ["MIOTA", 0.004333333],
    ["ICX", 0.004333333],
    ["XEM", 0.004333333],
    # ["UMA", 0.004333333],
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
