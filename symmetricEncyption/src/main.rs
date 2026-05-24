use aes::Aes256;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
mod encryption;
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

fn main() {
    encryption::encription();
}