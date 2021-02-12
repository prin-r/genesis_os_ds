#!/usr/bin/env python3

import json
import urllib.request
import sys


WAVES_MAINNET_URL = "https://nodes.wavesnodes.com/assets/balance/{}/{}"


def make_json_request(url):
    return json.loads(urllib.request.urlopen(url).read())


def main(asset_id, owner_address):
    res = make_json_request(WAVES_MAINNET_URL.format(owner_address, asset_id))
    return res.get("balance", None)


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
