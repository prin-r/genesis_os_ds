#!/usr/bin/env python3

import sys
import json
import requests

HEADERS = {"Content-Type": "application/json"}

SYMBOLS_WITH_WEIGHTS = [
    ["BTC", 0.2597],
    ["ETH", 0.2208],
    ["BNB", 0.036359999999999996],
    ["EOS", 0.036359999999999996],
    ["XTZ", 0.036359999999999996],
    ["ATOM", 0.036359999999999996],
    ["YFI", 0.036359999999999996],
    ["BCH", 0.010992307692307692],
    ["TRX", 0.010992307692307692],
    ["HT", 0.010992307692307692],
    ["OKB", 0.010992307692307692],
    # ["ZIL", 0.010992307692307692],
    ["CRO", 0.010992307692307692],
    ["XLM", 0.010992307692307692],
    ["ADA", 0.010992307692307692],
    ["DOT", 0.010992307692307692],
    ["SNX", 0.010992307692307692],
    ["ALGO", 0.010992307692307692],
    ["OMG", 0.010992307692307692],
    ["COMP", 0.010992307692307692],
    ["NEO", 0.007992307692307693],
    ["VET", 0.007992307692307693],
    ["BAT", 0.007992307692307693],
    ["AAVE", 0.007992307692307693],
    ["LINK", 0.007992307692307693],
    ["THETA", 0.007992307692307693],
    ["LEO", 0.007992307692307693],
    ["KNC", 0.007992307692307693],
    ["LTC", 0.007992307692307693],
    ["ZRX", 0.007992307692307693],
    ["XRP", 0.007992307692307693],
    ["MKR", 0.007992307692307693],
    # ["CELO", 0.007992307692307693],
    ["FTT", 0.016225],
    ["BTT", 0.016225],
    ["REN", 0.016225],
    ["QTUM", 0.016225],
    ["DGB", 0.0037142857142857142],
    ["ONT", 0.0037142857142857142],
    ["ICX", 0.0037142857142857142],
    ["KSM", 0.0037142857142857142],
    ["UMA", 0.0037142857142857142],
    ["XEM", 0.0037142857142857142],
    ["MIOTA", 0.0037142857142857142],
]


def main(url):
    symbols, weights = zip(*SYMBOLS_WITH_WEIGHTS)
    payload = {"symbols": symbols, "min_count": 10, "ask_count": 16}
    result = requests.request("POST", url, headers=HEADERS, json=payload).json()
    prices = [float(px["px"]) / float(px["multiplier"]) for px in result["result"]]
    acc = 0
    for (p, w) in zip(prices, weights):
        acc += float(p) * w
    return acc


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
