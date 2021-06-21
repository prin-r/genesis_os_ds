#!/usr/bin/env python3
import ccxt
import sys


def main(base_timeframe_limit, quote):
    base, timeframe, limit = base_timeframe_limit.split("_")
    binance = ccxt.binance()
    res = binance.fetch_ohlcv(symbol=f"{base}/{quote}", timeframe=timeframe, limit=int(limit, 0))
    if not res:
        raise ValueError("Got falsy value while getting price: " + str(res))

    avg = 0.0
    # open, high, low, close
    for _timestamp, o, h, l, c, _volume in res:
        avg += (o + h + l + c) / 4.0

    return avg / len(res)


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
