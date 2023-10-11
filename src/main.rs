use aes_gcm::{aed::OsRng, Aes256Gcm, KeyInit};
use base64::Engine;

fn main() {
    let key = Aes256Gcm::generate_key(&mut OsRng);

    let base64_engine = base64::engine::general_purpose::STANDARD;

    let secret_key = base64_engine.encode(key);

    println!("private key: {}", secret_key);
}
