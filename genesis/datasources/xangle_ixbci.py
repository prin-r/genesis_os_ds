#!/usr/bin/env python3

import sys
import json
import urllib.request
from datetime import datetime, timedelta

XANGLE = "https://pro-api.xangle.io/v1/index/xangle-largecap"


def make_json_request(url):
    req = urllib.request.Request(url)
    req.add_header("X-XANGLE_API_KEY", "64b21a5a-ba76-86e0-2dcbfd9be021")
    return json.loads(urllib.request.urlopen(req).read())


def main():
    now = datetime.utcnow()
    timestamp = now.strftime("%Y-%m-%dT%H:%M:%S")
    res = make_json_request(f"{XANGLE}?reference_timestamp={timestamp}")
    return res["data"]["index_value"]


if __name__ == "__main__":
    try:
        print(main())
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
