DIR=/Users/beeb/genesis_ds_os/genesis/scripts
bandd add-data-source \
	"CoinGecko Cryptocurrency Price" \
	"Retrieves current price of a cryptocurrency from https://www.coingecko.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/coingecko_price.py

bandd add-data-source \
	"CryptoCompare Cryptocurrency Price" \
	"Retrieves current price of a cryptocurrency from https://www.cryptocompare.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/crypto_compare_price.sh

bandd add-data-source \
	"Binance Cryptocurrency Price" \
	"Retrieves current price of a cryptocurrency from https://www.binance.com/en" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/binance_price.sh

bandd add-data-source \
	"Open Weather Map Weather Data" \
	"Retrieves current weather information from https://www.openweathermap.org" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/open_weather_map.sh

bandd add-data-source \
	"FreeForexAPI Gold Price" \
	"Retrives current gold price from https://www.freeforexapi.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/gold_price.sh

bandd add-data-source \
	"Alpha Vantage Stock Price" \
	"Retrives current price of a stock from https://www.alphavaage.co" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/alphavantage.sh

bandd add-data-source \
	"Blockchain.info Bitcoin Block Count" \
	"Retrives latest Bitcoin block height from https://www.blockchain.info" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/bitcoin_count.sh

bandd add-data-source \
	"BlockCypher Bitcoin Block Hash" \
	"Retrives Bitcoin block hash at a given block height from https://blockcypher.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/bitcoin_hash.sh

bandd add-data-source \
	"CoinGecko Cryptocurrency Trading Volume" \
	"Retrieves current trading volume of a cryptocurrency from https://www.coingecko.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/coingecko_volume.sh

bandd add-data-source \
	"CryptoCompare Cryptocurrency Trading Volume" \
	"Retrieves current trading volume of a cryptocurrency from https://www.cryptocompare.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/crypto_compare_volume.sh

bandd add-data-source \
	"ETH Gas Station Current Ethereum Gas Price" \
	"Retrieves current Ethereum gas price from https://ethgasstation.info" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/ethgasstation.sh

bandd add-data-source \
	"Open Sky Network Flight Data" \
	"Retrieves flight information from https://opensky-network.org" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/open_sky_network.sh

bandd add-data-source \
	"Quantum Random Number Generator" \
	"Retrieves array of random number from https://qrng.anu.edu.au" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/qrng_anu.sh

bandd add-data-source \
	"Yahoo Finance Stock Price" \
	"Retrieves current price of a stock from https://finance.yahoo.com" \
	band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs \
	$DIR/../datasources/yahoo_finance.py

