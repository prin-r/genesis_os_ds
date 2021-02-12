#!/usr/bin/env python3
import ccxt
import requests
import sys


def gdac(base_symbol, quote_symbol):
    session = requests.session()
    result = session.get(
        f"https://partner.gdac.com/v0.4/public/orderbook?pair={base_symbol}%2F{quote_symbol}",
        timeout=4,
    ).json()
    askprice = float(result["ask"][0]["price"])
    bidprice = float(result["bid"][0]["price"])

    return f"{askprice},{bidprice}"


def gopax(base_symbol, quote_symbol):
    session = requests.session()
    result = session.get(
        f"https://api.gopax.co.kr/trading-pairs/{base_symbol}-{quote_symbol}/book", timeout=4
    ).json()
    askprice = float(result["ask"][0][1])
    bidprice = float(result["bid"][0][1])

    return f"{askprice},{bidprice}"


other_exchanges = {"gopax": gopax, "gdac": gdac}


def main(exchange, base_symbol, quote_symbol):
    if hasattr(ccxt, exchange):
        ex = getattr(ccxt, exchange)()
        if not ex.has["fetchOrderBook"]:
            raise Exception(
                f"CCXT Error: exchange {exchange} doesn't have function fetch_order_book"
            )
        orders = ex.fetch_order_book(f"{base_symbol}/{quote_symbol}")

        askprice = float(orders["asks"][0][0])
        bidprice = float(orders["bids"][0][0])

        return f"{askprice},{bidprice}"
    elif exchange in other_exchanges:
        return other_exchanges[exchange](base_symbol, quote_symbol)

    raise Exception(f"Error: Exchange {exchange} not found")


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
