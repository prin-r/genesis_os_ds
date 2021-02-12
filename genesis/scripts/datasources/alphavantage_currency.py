#!/usr/bin/env python3

import json
import urllib.request
import sys

ALPHA_VANTAGE_URL = "https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey=2KMCBLAZM5TE2VPG"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(currency_1, currency_2):
    res = make_json_request(ALPHA_VANTAGE_URL.format(currency_1, currency_2))
    if "Realtime Currency Exchange Rate" not in res:
        raise ValueError("key 'Realtime Currency Exchange Rate' not found")
    if "5. Exchange Rate" not in res["Realtime Currency Exchange Rate"]:
        raise ValueError("key '5. Exchange Rate' not found")

    return res["Realtime Currency Exchange Rate"]["5. Exchange Rate"]


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
