#!/usr/bin/env python3

import sys
import re
from datetime import datetime, timezone
import urllib.request
from websocket import create_connection
import json


DERIBIT_URL_WS = "wss://www.deribit.com/ws/api/v2"
DERIBIT_URL_REST = "https://www.deribit.com/api/v2/public/get_index_price?index_name={}"

DIFF_MARKET_PRICE_PERCENTAGE = 0.05


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(symbol):
    res = make_json_request(DERIBIT_URL_REST.format(f"{symbol.lower()}_usd"))
    current_price = res["result"]["index_price"]

    ws = create_connection(DERIBIT_URL_WS)
    ws.send(
        json.dumps(
            {
                "jsonrpc": "2.0",
                "method": "public/subscribe",
                "id": 42,
                "params": {
                    "channels": [
                        f"markprice.options.{symbol.lower()}_usd",
                    ]
                },
            }
        )
    )
    result = {}
    while "params" not in result:
        result = ws.recv()

    ws.close()

    ivs = json.loads(result)["params"]["data"]
    avg = 0
    iv_num = 0
    for iv in ivs:
        [_symbol, timestamp_str, price_usd, _] = iv["instrument_name"].split("-")
        [day, year] = re.split(r"\D+", timestamp_str)
        [month] = re.findall(r"\D+", timestamp_str)

        iv_time = int(
            datetime.strptime(f"{day} {month} {year} 22:00:00", "%d %b %y %H:%M:%S")
            .replace(tzinfo=timezone.utc)
            .timestamp()
        )
        cur_time = int(datetime.utcnow().timestamp())

        if iv_time < cur_time + (60 * 60 * 24 * 7):
            if abs(current_price - float(price_usd)) / current_price < DIFF_MARKET_PRICE_PERCENTAGE:
                avg += iv["iv"]
                iv_num += 1

    return avg / iv_num


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
