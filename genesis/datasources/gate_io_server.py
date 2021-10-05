from flask import Flask, Response, request, jsonify
import requests

app = Flask(__name__)

URL = "https://laozi-testnet4.bandchain.org/api/oracle/v1/request_prices"


valid_keys = ["9W7037BXF3SNB9DN", "O500ZF2PDSXJF1VW", "1BJTSADLCRAU2ME8"]


@app.route("/")
def fake_endpoint():
    try:
        api_key = request.authorization["password"]
        if api_key not in valid_keys:
            return Response(
                "access is denied",
                401,
                {"WWW-Authenticate": 'Basic realm="Invalid API-Key"'},
            )
        symbols = request.args.getlist("symbols")

        r = requests.get(
            URL,
            params={
                "symbols": symbols,
                "ask_count": 4,
                "min_count": 3,
            },
        )

        rj = r.json()

        if "price_results" not in rj:
            return rj

        pr = rj["price_results"]

        return jsonify([int(i["px"]) / int(i["multiplier"]) for i in pr])
    except Exception as e:
        return str(e)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)
