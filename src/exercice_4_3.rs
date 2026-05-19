/* 
    Computer Security TV1 - Library for Exercice 4.3
    Public Key Encryption Algorithm: RSA
    Digital Signature System: ED25519
    Author: Hugo Donaire
*/

use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use rsa::sha2::Sha256;
use rand::thread_rng;
use rand::rngs::OsRng;
use ed25519_dalek::{
    SigningKey,
    VerifyingKey,
    Signature,
    Signer,
    Verifier,
};

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

pub fn ed25519_key_generation() -> (SigningKey, VerifyingKey) {
    let mut rng = OsRng;

    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    (signing_key, verifying_key)
}

pub fn ed25519_sign(priv_key: &SigningKey, text: &[u8]) -> Signature {
    priv_key.sign(text)
}

pub fn ed25519_verification(pub_key: &VerifyingKey, text: &[u8], signature: &Signature) -> bool {
    pub_key.verify(text, signature).is_ok()
}
