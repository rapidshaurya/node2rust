
use rsa::{Oaep};
use rsa::{pkcs8::DecodePublicKey, PublicKey, RsaPublicKey};

const RSA_2048_PUB_DER: &[u8] = include_bytes!("../pub_key.der");
fn main() {
    let mut rng = rand::thread_rng();
    let padding = Oaep::new::<sha1::Sha1>();
    let message = format!("write message here");
   
    let rsa_pub_key = RsaPublicKey::from_public_key_der(RSA_2048_PUB_DER).unwrap();
    let mess = rsa_pub_key
        .encrypt(&mut rng, padding, message.as_bytes())
        .unwrap();
    println!("{:?}", mess);

}