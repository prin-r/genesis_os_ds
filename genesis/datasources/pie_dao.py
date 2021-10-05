#!/usr/bin/env python3

import sys
import requests

HEADERS = {"Content-Type": "application/json"}

SYMBOLS_WITH_WEIGHTS = [
    ["LINK", 0.02096283656],
    ["COMP", 0.0007175048356],
    ["MKR", 0.0001197624459],
    ["SNX", 0.02484347627],
    ["UNI", 0.02029552982],
    ["SUSHI", 0.02615047318],
    ["AAVE", 0.001708902225],
    ["YFI", 0.000006385948438],
    ["UMA", 0.03597233199],
    ["REN", 0.3406238814],
    ["LRC", 0.698313375],
    ["BAL", 0.006502306806],
    ["PNT", 0.01800566292],
    ["MLN", 0.0005813148685],
]


def main(url):
    symbols, weights = zip(*SYMBOLS_WITH_WEIGHTS)
    result = requests.get(
        url,
        params={"symbols": symbols, "min_count": 10, "ask_count": 16},
    ).json()
    prices = [float(px["px"]) / float(px["multiplier"]) for px in result["price_results"]]
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
