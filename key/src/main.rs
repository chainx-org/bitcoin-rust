extern crate keys;

use keys::generator::Generator;
use keys::{Private, KeyPair};

fn main() {
    let random = keys::generator::Random::new(keys::Network::Testnet);
    let key = random.generate().unwrap();
    println!("{:?}", key);
    let private = key.private();
    let compressed_private = Private{
        network: private.network.clone(),
        secret: private.secret.clone(),
        compressed: true,
    };
    let compressed = KeyPair::from_private(compressed_private).unwrap();
    println!("compressed public key: {}",compressed.public());
    println!("address:{:?}", key.address().to_string());
}
