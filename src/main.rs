extern crate gdax_client;

use std::time::Duration;
use std::thread;

use gdax_client::{Error, NewOrder, PrivateClient, PublicClient, Side, SizeOrFunds};

/// How many $USD to exchange for BTC per transaction.
const AMT_BTC_USD: f64 = 0.0228155;
/// How many $USD to exchange for ETH per transaction.
const AMT_ETH_USD: f64 = 0.0114078;
/// How frequently to transact, in seconds
const DELAY_SECS: u64 = 60;
/// How long to wait before retry when experiencing HTTP errors, in seconds
const HTTP_RETRY_SECS: u64 = 30;

const CB_KEY: &'static str = include_str!("../api.key");
const CB_SECRET: &'static str = include_str!("../api.secret");
const CB_PASSPHRASE: &'static str = include_str!("../api.passphrase");

//const CURRENCIES: [&'static str; 3] = ["BTC", "ETH", "LTC"];
const PRODUCTS: [&'static str; 3] = ["BTC-USD", "ETH-USD", "LTC-USD"];

fn main() {
    let cb_key = CB_KEY.trim();
    let cb_secret = CB_SECRET.trim();
    let cb_pass = CB_PASSPHRASE.trim();
    let public_client = PublicClient::new();
    let private_client = PrivateClient::new(cb_key, cb_secret, cb_pass);
    private_client.get_accounts().unwrap();

    let order_btc = NewOrder::market(Side::Buy, "BTC-USD", SizeOrFunds::Funds(AMT_BTC_USD));
    let order_eth = NewOrder::market(Side::Buy, "ETH-USD", SizeOrFunds::Funds(AMT_ETH_USD));
    let orders = [order_btc, order_eth];

    loop {
        let accounts = private_client.get_accounts().unwrap();
        print!("BAL: ");
        for acc in &accounts {
            print!("{:.10} {}    ", acc.available, acc.currency);
        }
        println!("");
        print!("RATE: ");
        for product in &PRODUCTS {
            let tick = public_client.get_product_ticker(product).unwrap();
            print!("{:.7} {}    ", tick.price, product);
        }
        println!("");
        for order in &orders {
            println!("Posting market order: {:?}", order);
            match private_client.post_order(&order) {
                Ok(ok) => {
                    println!("  -> {}", ok);
                },
                Err(Error::Http(err)) => {
                    // Http error; could indicate temporary network outage
                    println!("  -> {:?}", err);
                    thread::sleep(Duration::from_secs(HTTP_RETRY_SECS));
                },
                Err(other) => {
                    println!("  -> {:?}", other);
                    panic!("Error posting order: {:?}", other);
                }
            }
        }
        thread::sleep(Duration::from_secs(DELAY_SECS));
    }

}
