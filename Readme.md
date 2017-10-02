## About
This is a very simple bot that posts a BTC buy order for a fixed amount of $USD on [GDAX](https://gdax.com), sleeps for some time, and
then repeats. The idea is to steadily purchase bitcoin while minimizing the effects of volatility. Some people refer to this as "dollar cost averaging", but if you pride yourself on knowing all the financial lingo you can shove it.

## Configuration
First, create three files at the root repository and populate them based on your GDAX API values: `api.key`, `api.secret` and `api.password` (whitespace will automatically be stripped). Then edit the constants at the top of src/main.rs to suit your needs. For the time being, the API keys actually get compiled into the program, so you probably don't want to distribute the binary.

Although the GDAX web interface only displays USD balances to 2 decimal places, it seems to track it internally to _far_ more places. The effect of this is that you can be a bit more precise in your purchase amounts.

On the other hand the amount of USD specified doesn't exactly match the amount of USD actually spent since GDAX _does_ force the BTC value to be a multiple of 0.00000001. So over the course of a month, you might exchange a few dollars more or less than simple math would have predicted. Furthermore, the pacing is implemented by `sleep`ing for a constant, which means network or CPU contention will also impact the rate at which funds are exchanged.

## License
MIT/BSD. I don't care what you do with this, but **use at your own risk**.

