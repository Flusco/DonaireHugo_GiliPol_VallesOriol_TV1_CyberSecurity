use std::io;
use crate::exercice_4_3;

fn col_alph_order(key: &str) -> Vec<usize> {
    let mut cols: Vec<(usize, char)> = key.chars().enumerate().collect();
    cols.sort_by_key(|&(index, ch)| (ch, index));
    cols.into_iter().map(|(index, _)| index).collect()
}

fn columnar_cipher(key: &str, text: &str) -> String {
    let cols = key.chars().count();
    let mut chars: Vec<char> = text.chars().collect();
    let rows = (chars.len() + cols - 1) / cols;

    chars.resize(rows * cols, 'X');

    let order = col_alph_order(key);
    let mut result = String::new();

    for col in order {
        for row in 0..rows {
            let position = row * cols + col;
            result.push(chars[position]);
        }
    }

    result
}

fn columnar_decipher(key: &str, text: &str) -> String {
    let cols = key.chars().count();
    let chars: Vec<char> = text.chars().collect();
    let rows = chars.len() / cols;
    let order = col_alph_order(key);
    let mut grid = vec!['X'; rows * cols];
    let mut text_position = 0;

    for col in order {
        for row in 0..rows {
            let grid_position = row * cols + col;
            grid[grid_position] = chars[text_position];
            text_position += 1;
        }
    }

    grid.into_iter().collect()
}

fn caesar_cipher(key: u8, text: &str) -> String {
    let mut result = String::new();

    for ch in text.chars() {
        if ch.is_ascii_uppercase() {
            let new_char = ((ch as u8 - b'A' + key) % 26) + b'A';
            result.push(new_char as char);
        } else if ch.is_ascii_lowercase() {
            let new_char = ((ch as u8 - b'a' + key) % 26) + b'a';
            result.push(new_char as char);
        } else {
            result.push(ch);
        }
    }

    result
}

fn caesar_decipher(key: u8, text: &str) -> String {
    let reverse_key = 26 - (key % 26);
    caesar_cipher(reverse_key, text)
}

fn crypto_bronium_copy(k1: &str, k2: &str, k3: u8, text: &str) -> String {
    let first_round = columnar_cipher(k1, text);
    let second_round = columnar_cipher(k2, &first_round);
    caesar_cipher(k3, &second_round)
}

fn decrypto_bronium_copy(k1: &str, k2: &str, k3: u8, text: &str) -> String {
    let caesar_removed = caesar_decipher(k3, text);
    let second_transposition_removed = columnar_decipher(k2, &caesar_removed);
    columnar_decipher(k1, &second_transposition_removed)
}

fn read_text() -> String {
    println!("\nWrite a message for the cryptanalysis test:");

    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read text");

    let message = message.trim().to_string();

    if message.is_empty() {
        String::from("The secret password is hidden in the server room")
    } else {
        message
    }
}

fn remove_padding(text: &str) -> String {
    text.trim_end_matches('X').to_string()
}

fn preview(text: &str) -> String {
    let clean_text = remove_padding(text);
    let mut short_text = String::new();

    for ch in clean_text.chars().take(90) {
        short_text.push(ch);
    }

    if clean_text.chars().count() > 90 {
        short_text.push_str("...");
    }

    short_text
}

fn english_score(text: &str) -> i32 {
    let lower = text.to_lowercase();
    let mut score = 0;

    for word in [" the ", " and ", " is ", " in ", " to ", " secret", " password", " attack", " message"] {
        if lower.contains(word) {
            score += 8;
        }
    }

    for ch in lower.chars() {
        if ch == ' ' {
            score += 2;
        } else if "etaoinshrdlu".contains(ch) {
            score += 1;
        } else if ch.is_ascii_digit() {
            score += 1;
        } else if ch == 'x' {
            score -= 1;
        }
    }

    score
}

pub fn cryptanalysis_4_1() {
    println!("\n=========== EXERCISE 4.6: CRYPTANALYSIS OF 4.1 ===========");

    let k1 = "MANGO";
    let k2 = "TEBAS";
    let real_caesar_key = 3;
    let message = read_text();
    let cipher_text = crypto_bronium_copy(k1, k2, real_caesar_key, &message);

    println!("\nPlaintext used for the test:");
    println!("{}", message);
    println!("\nCiphertext produced with the 4.1 style cipher:");
    println!("{}", cipher_text);

    println!("\nAttack 1: brute force the Caesar layer only.");
    println!("There are only 26 possible Caesar keys, so this part is very easy to try.\n");

    for key in 0..26 {
        let caesar_removed = caesar_decipher(key, &cipher_text);
        println!("key {:2}: {}", key, preview(&caesar_removed));
    }

    println!("\nAttack 2: if the transposition keys are known or guessed, test every Caesar key.");
    println!("This is useful because the keys MANGO and TEBAS are fixed in the demo.\n");

    let mut results: Vec<(u8, i32, String)> = Vec::new();

    for key in 0..26 {
        let candidate = decrypto_bronium_copy(k1, k2, key, &cipher_text);
        let score = english_score(&candidate);
        results.push((key, score, candidate));
    }

    results.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Best candidates according to a simple English score:\n");

    for (key, score, candidate) in results.iter().take(5) {
        println!("key {:2}, score {:3}: {}", key, score, preview(candidate));
    }

    println!("\nReal Caesar key used by the program: {}", real_caesar_key);
    println!("If the best candidate is readable, the attack worked.");

    println!("\nWeaknesses found:");
    println!("- The Caesar part has only 26 possible keys.");
    println!("- The transposition keys are words, so they can be guessed or leaked.");
    println!("- Reusing the same keys makes attacks easier.");
    println!("- This cipher is good for learning, but it is not secure for real use.");

    println!("\nPossible improvements:");
    println!("- Replace the custom cipher with AES-GCM or another trusted modern cipher.");
    println!("- Use random nonces/IVs when needed.");
    println!("- Never hardcode or reuse weak keys.");
}

pub fn cryptanalysis_4_3() {
    println!("\n=========== EXERCISE 4.6: CRYPTANALYSIS OF 4.3 ===========");

    let mut message = read_text();

    if message.as_bytes().len() > 180 {
        println!("\nThe message is too long for the RSA demo, so a shorter default message is used.");
        message = String::from("Short message for RSA cryptanalysis");
    }

    println!("\nTarget: RSA encryption with OAEP and Ed25519 signatures.");
    println!("Plaintext used for the test:");
    println!("{}", message);

    println!("\nDirect decryption attempt:");
    println!("Without Bob's private RSA key, the attacker cannot normally decrypt the ciphertext.");
    println!("Instead of pretending to break RSA, we show the real weak point: unverified public keys.\n");

    exercice_4_3::communication_demo(message.as_bytes());

    println!("\nNow we reuse the Man-in-the-Middle test from Exercise 4.3.");
    println!("This shows that the cryptography can be strong but the key exchange can still fail.\n");

    exercice_4_3::mitm_attack_demo(message.as_bytes());

    println!("\nWeaknesses found:");
    println!("- RSA-OAEP and Ed25519 are not the easy part to break.");
    println!("- The dangerous point is trusting the wrong public key.");
    println!("- If Alice encrypts with Mallory's public key, Mallory can read the message.");
    println!("- If Bob trusts Mallory's verifying key, Bob can accept a fake message.");

    println!("\nPossible improvements:");
    println!("- Verify public keys before using them.");
    println!("- Use certificates, fingerprints, or a trusted key server.");
    println!("- Bind identities to public keys.");
    println!("- Keep private keys secret and protected.");
}

pub fn full_cryptanalysis() {
    cryptanalysis_4_1();
    cryptanalysis_4_3();
}
