/* 
    Computer Security TV1 - Main Program
    Implemented Exercices: 4.1, 4.3
    Author: Hugo Donaire, Pol Gili, Oriol Vallès
*/

mod exercice_4_3;

use exercice_4_3::{
    rsa_key_generation,
    rsa_encrypt,
    rsa_decrypt,
};

fn main() {
    let (priv_key, pub_key) = rsa_key_generation();

    let message = b"hello world";

    let encrypted = rsa_encrypt(&pub_key, message);
    let decrypted = rsa_decrypt(&priv_key, &encrypted);

    println!("Original:  {:?}", String::from_utf8_lossy(message));
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));

    assert_eq!(message, &decrypted[..]);
}