use aes::Aes256;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

use cbc::{Decryptor as CbcDecryptor, Encryptor as CbcEncryptor};
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};

use cipher::{
    block_padding::Pkcs7,
    KeyInit as CipherKeyInit,
    KeyIvInit,
    StreamCipher,
};
use cipher::BlockModeEncrypt;
use cipher::BlockModeDecrypt;
use rand::{Rng, rng};
use rc4::Rc4;
use std::time::Instant;

pub fn encription() {
    let plaintext = b"Confidential message between Alice and Bob.";

    let mut key = [0u8; 32];
    rng().fill_bytes(&mut key);

    let mut iv = [0u8; 16];
    rng().fill_bytes(&mut iv);

    // =====================================
    // AES ECB
    // =====================================
    println!("===== AES-256 ECB =====");

    let start = Instant::now();

    let cipher_ecb = EcbEncryptor::<Aes256>::new(&key.into());

    let mut buffer = plaintext.to_vec();
    let msg_len = buffer.len();

    buffer.resize(msg_len + 16, 0);

    let ciphertext = cipher_ecb
        .encrypt_padded::<Pkcs7>(&mut buffer, msg_len)
        .unwrap();

    let ciphertext_ecb = ciphertext.to_vec();

    println!("Ciphertext: {}", hex::encode(&ciphertext_ecb));

    let encrypt_time = start.elapsed();

    let start = Instant::now();

    let cipher_ecb_dec = EcbDecryptor::<Aes256>::new(&key.into());

    let mut decrypt_buf = ciphertext_ecb.clone();

    let decrypted = cipher_ecb_dec
        .decrypt_padded::<Pkcs7>(&mut decrypt_buf)
        .unwrap();

    println!("Decrypted: {}", String::from_utf8_lossy(decrypted));

    let decrypt_time = start.elapsed();

    println!("Encrypt Time: {:?}", encrypt_time);
    println!("Decrypt Time: {:?}\n", decrypt_time);

    // =====================================
    // AES CBC
    // =====================================
    println!("===== AES-256 CBC =====");

    let start = Instant::now();

    let cipher_cbc = CbcEncryptor::<Aes256>::new(&key.into(), &iv.into());

    let mut buffer = plaintext.to_vec();
    let msg_len = buffer.len();

    buffer.resize(msg_len + 16, 0);

    let ciphertext = cipher_cbc
        .encrypt_padded::<Pkcs7>(&mut buffer, msg_len)
        .unwrap();

    let ciphertext_cbc = ciphertext.to_vec();

    println!("Ciphertext: {}", hex::encode(&ciphertext_cbc));

    let encrypt_time = start.elapsed();

    let start = Instant::now();

    let cipher_cbc_dec = CbcDecryptor::<Aes256>::new(&key.into(), &iv.into());

    let mut decrypt_buf = ciphertext_cbc.clone();

    let decrypted = cipher_cbc_dec
        .decrypt_padded::<Pkcs7>(&mut decrypt_buf)
        .unwrap();

    println!("Decrypted: {}", String::from_utf8_lossy(decrypted));

    let decrypt_time = start.elapsed();

    println!("Encrypt Time: {:?}", encrypt_time);
    println!("Decrypt Time: {:?}\n", decrypt_time);

    // =====================================
    // AES GCM
    // =====================================
    println!("===== AES-256 GCM =====");

    let cipher_gcm = Aes256Gcm::new_from_slice(&key).unwrap();

    let mut nonce_bytes = [0u8; 12];
    rng().fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let start = Instant::now();

    let ciphertext_gcm = cipher_gcm.encrypt(nonce, plaintext.as_ref()).unwrap();

    let encrypt_time = start.elapsed();

    println!("Ciphertext: {}", hex::encode(&ciphertext_gcm));

    let start = Instant::now();

    let decrypted_gcm = cipher_gcm.decrypt(nonce, ciphertext_gcm.as_ref()).unwrap();

    let decrypt_time = start.elapsed();

    println!(
        "Decrypted: {}",
        String::from_utf8_lossy(&decrypted_gcm)
    );

    println!("Encrypt Time: {:?}", encrypt_time);
    println!("Decrypt Time: {:?}\n", decrypt_time);

    // =====================================
    // RC4
    // =====================================
    println!("===== RC4 =====");

    let rc4_key = b"supersecretkey";

    let start = Instant::now();

    let mut rc4_enc = Rc4::new_from_slice(rc4_key).unwrap();

    let mut rc4_ciphertext = plaintext.to_vec();

    rc4_enc.apply_keystream(&mut rc4_ciphertext);

    let encrypt_time = start.elapsed();

    println!("Ciphertext: {}", hex::encode(&rc4_ciphertext));

    let start = Instant::now();

    let mut rc4_dec = Rc4::new_from_slice(rc4_key).unwrap();

    let mut decrypted = rc4_ciphertext.clone();

    rc4_dec.apply_keystream(&mut decrypted);

    let decrypt_time = start.elapsed();

    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));

    println!("Encrypt Time: {:?}", encrypt_time);
    println!("Decrypt Time: {:?}", decrypt_time);
}