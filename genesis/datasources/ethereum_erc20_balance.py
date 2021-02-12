#!/usr/bin/env python3

import json
import urllib.request
import sys


ETH_MAINNET_URL = "https://mainnet.infura.io/v3/8a0f2143b6444ee0a8d0f0414fd533d2"


def make_json_request(req):
    return json.loads(urllib.request.urlopen(req).read())


def main(erc20_address, holder_address):
    if erc20_address[:2] != "0x":
        erc20_address = "0x" + erc20_address

    if holder_address[:2] == "0x":
        holder_address = holder_address[2:]

    req = urllib.request.Request(
        ETH_MAINNET_URL,
        data=json.dumps(
            {
                "jsonrpc": "2.0",
                "method": "eth_call",
                "params": [
                    {"to": erc20_address, "data": "0x70a08231" + holder_address.zfill(64)},
                    "latest",
                ],
                "id": 1,
            }
        ).encode("utf-8"),
    )
    res = make_json_request(req)
    if "error" in res:
        raise ValueError(str(res["error"]))

    return int(res["result"][2:], 16)


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
