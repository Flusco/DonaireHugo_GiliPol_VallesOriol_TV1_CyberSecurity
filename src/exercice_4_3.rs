/* 
    Computer Security TV1 - Library for Exercice 4.3
    Public Key Encryption Algorithm: RSA
    Digital Signature System: ED25519
    Secure Communication Protocol:
    Author: Hugo Donaire
*/

use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use rsa::sha2::Sha256;
use rand::thread_rng;

pub fn rsa_key_generation () -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate key");
    let pub_key = RsaPublicKey::from(&priv_key);

    (priv_key, pub_key)
}

pub fn rsa_encrypt (pub_key: &RsaPublicKey, text: &[u8]) -> Vec<u8> {
    let mut rng = thread_rng();

    pub_key.encrypt(&mut rng, Oaep::new::<Sha256>(), text).expect("Failed to encrypt.")
}

pub fn rsa_decrypt (priv_key: &RsaPrivateKey, text: &[u8]) -> Vec<u8> {
    priv_key.decrypt(Oaep::new::<Sha256>(), text).expect("Failed to decrypt.")
}

