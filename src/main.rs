use rand::*;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    println!("{:?}", priv_key);
    println!();
    println!("{:?}", pub_key);
}
