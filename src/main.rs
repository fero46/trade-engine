mod market;

fn main() {
    let mut manager = market::manager::Manager::new();
    manager.buildMarket(String::from("BTCUSD"));
}
