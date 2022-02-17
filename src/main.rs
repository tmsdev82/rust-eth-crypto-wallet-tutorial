mod eth_wallet;
fn main() {
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());
}
