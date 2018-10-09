extern crate keys;

use keys::generator::Generator;

fn main() {
    let random = keys::generator::Random::new(keys::Network::Testnet);
    let key = random.generate().unwrap();
    println!("{:?}", key);
    println!("address:{:?}", key.address().to_string());
}
