import requests
from requests.auth import HTTPBasicAuth

valid_keys = ["9W7037BXF3SNB9DN", "O500ZF2PDSXJF1VW", "1BJTSADLCRAU2ME8"]


def query():
    api_key = "1BJTSADLCRAU2ME1"

    resp = requests.get(
        "http://128.199.144.100:5000/",
        # "http://127.0.0.1:5000",
        params={
            "symbols": ["BTC", "ETH", "BAND", "MIR"],
        },
        auth=HTTPBasicAuth("API Key", api_key),
    )

    print(resp)
    print(resp.text)

    return resp.json(), resp.status_code


print(query())
