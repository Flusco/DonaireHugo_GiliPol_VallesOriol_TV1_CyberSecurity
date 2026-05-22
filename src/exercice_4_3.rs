/* 
    Computer Security TV1 - Library for Exercice 4.3
    Public Key Encryption Algorithm: RSA
    Digital Signature System: ED25519
    Author: Hugo Donaire, Oriol Vallès, Pol Gili
*/


use rand::RngCore;
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use rsa::sha2::Sha256;
use rand::rngs::OsRng;
use ed25519_dalek::{
    SigningKey,
    VerifyingKey,
    Signature,
    Signer,
    Verifier,
};

pub struct SecureMessage {
    pub nonce: Vec<u8>,
    pub cipher_text: Vec<u8>,
    pub signature: Signature,
}

fn rsa_key_generation () -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate key");
    let pub_key = RsaPublicKey::from(&priv_key);

    (priv_key, pub_key)
}

fn rsa_encrypt (pub_key: &RsaPublicKey, text: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;

    pub_key.encrypt(&mut rng, Oaep::new::<Sha256>(), text).expect("Failed to encrypt.")
}

fn ed25519_key_generation() -> (SigningKey, VerifyingKey) {
    let mut rng = OsRng;

    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    (signing_key, verifying_key)
}

fn ed25519_sign(priv_key: &SigningKey, text: &[u8]) -> Signature {
    priv_key.sign(text)
}

fn ed25519_verification(pub_key: &VerifyingKey, text: &[u8], signature: &Signature) -> bool {
    pub_key.verify(text, signature).is_ok()
}

fn build_sign_data (nonce: &[u8], text :&[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(nonce);
    data.extend_from_slice(text);
    data
}

fn generate_nonce () -> Vec<u8> {
    let mut nonce = vec![0u8;16];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

fn create_secure_message (public_key: &RsaPublicKey, signing_key: &SigningKey, text: &[u8]) -> SecureMessage {
    let nonce = generate_nonce();

    let cipher_text = rsa_encrypt(public_key, text);
    let sign_data = build_sign_data(&nonce, text);
    let signature = ed25519_sign(signing_key, &sign_data);

    SecureMessage {nonce,cipher_text,signature}
}

fn open_secure_message (private_key: &RsaPrivateKey, verifying_key: &VerifyingKey, secure_message: &SecureMessage) -> Option<Vec<u8>> {
    let text = match private_key.decrypt(
        Oaep::new::<Sha256>(),
        &secure_message.cipher_text
    ){
        Ok(text) => text,
        Err(_) => return None
    };

    let sign_data = build_sign_data(&secure_message.nonce, &text);

    if !ed25519_verification(verifying_key, &sign_data, &secure_message.signature){
        return None;
    }

    Some(text)
}

fn print_result(title: &str, result: Option<Vec<u8>>) {
    println!("\n{}", title);

    match result {
        Some(text) => println!("Accepted message: {}", String::from_utf8_lossy(&text)),
        None => println!("Rejected message: decryption or signature verification failed."),
    }
}

pub fn communication_demo(text: &[u8]) {
    println!("===========DEMO COMMUNICATION===========");

    let (alice_signing_key, alice_verifying_key) = ed25519_key_generation();
    let (bob_rsa_private_key, bob_rsa_public_key) = rsa_key_generation();

    println!("Alice encrypts with Bob's public key.");
    println!("Alice signs the message with her own Ed25519 private key");

    let secure_message = create_secure_message (&bob_rsa_public_key, &alice_signing_key, text);

    println!("Cipher size: {} bytes", secure_message.cipher_text.len());
    println!("Nonce size: {} byres", secure_message.nonce.len());
    println!("Signature size {} bytes", secure_message.signature.to_bytes().len());

    let result = open_secure_message(&bob_rsa_private_key, &alice_verifying_key, &secure_message);

    print_result("Bob verifies Alice's signatue and decrypts:", result);
}

pub fn mitm_attack_demo(text: &[u8]) {
    println!("===========MAN-IN-THE-MIDDLE===========");

    let (alice_signing_key, alice_verifying_key) = ed25519_key_generation();
    let (bob_rsa_private_key, bob_rsa_public_key) = rsa_key_generation();

    let (mallory_rsa_private_key, mallory_rsa_public_key) = rsa_key_generation();
    let (mallory_signing_key, mallory_verifying_key) = ed25519_key_generation();

    println!("Alice wants Bob's RSA public key.");
    println!("Malloy tries to intercept the exchange and sends her public key.");
    println!("Alice uses Mallory's public key and encrypts the message for Mallory :0 !!!");

    let message_mallory = create_secure_message(&mallory_rsa_public_key, &alice_signing_key, text);
    let secure_mallory = open_secure_message(&mallory_rsa_private_key, &alice_verifying_key, &message_mallory);

    print_result("Mallory is able to decrypt Alice's message for Bob!!!", secure_mallory);
    println!("Now Mallory creates a new message for Bob and signs it with her own key.");

    let forged_message = create_secure_message(&bob_rsa_public_key, &mallory_signing_key, text);
    let wrong_verification = open_secure_message(&bob_rsa_private_key, &mallory_verifying_key, &forged_message);

    print_result("If Bob trusts Mallory's key, he accepts:", wrong_verification);

    let true_verification = open_secure_message(&bob_rsa_private_key, &alice_verifying_key, &forged_message);

    print_result("If Bob uses Alice verifying key, he rejects:", true_verification);

    println!("In conclusion: USE A TRUSTED CHANNEL FOR KEY EXCHANGE!!!!");
}
