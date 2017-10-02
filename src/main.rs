extern crate gdax_client;

use std::time::Duration;
use std::thread;

use gdax_client::{NewOrder, PrivateClient, Side, SizeOrFunds};

/// How many $USD to exchange per transaction
const AMT_USD: f64 = 0.0228155;
/// How frequently to transact, in seconds
const DELAY_SECS: u64 = 60;

const CB_KEY: &'static str = include_str!("../api.key");
const CB_SECRET: &'static str = include_str!("../api.secret");
const CB_PASSPHRASE: &'static str = include_str!("../api.passphrase");

fn main() {
    let cb_key = CB_KEY.trim();
    let cb_secret = CB_SECRET.trim();
    let cb_pass = CB_PASSPHRASE.trim();
    let private_client = PrivateClient::new(cb_key, cb_secret, cb_pass);
    private_client.get_accounts().unwrap();

    let order = NewOrder::market(Side::Buy, "BTC-USD", SizeOrFunds::Funds(AMT_USD));

    loop {
        let accounts = private_client.get_accounts().unwrap();
        print!("BAL: ");
        for acc in &accounts {
            print!("{:.10} {}    ", acc.available, acc.currency);
        }
        println!("");
        println!("Posting market order: {:?} {:?}", order, private_client.post_order(&order));
        thread::sleep(Duration::from_secs(DELAY_SECS));
    }

}
