/* 
    Computer Security TV1 - Library for Exercice 4.1
    Transposition Cypher: Double Columnar Transpostion Cypher
    Substitution Cypher: Caesar Cypher
    Author: Hugo Donaire
*/

fn col_alph_order (key: &str)-> Vec<usize>{
    let mut cols: Vec<(usize, char)> = key.chars().enumerate().collect();
    cols.sort_by_key(|&(index,ch)|(ch,index));

    cols.into_iter().map(|(index,_)|index).collect()
}


fn columnar_cypher (k1: &str, to_cyph: &str) -> String{
    let cols = k1.chars().count();

    assert!(cols>0, "A key needs to be introduced.");

    let mut chars: Vec<char> = to_cyph.chars().collect();

    let rows = chars.len().div_ceil(cols);

    chars.resize(rows*cols,'X');

    let order = col_alph_order(k1);
    let mut cipher = String::with_capacity(chars.len());

    for &col in &order {
        for row in 0..rows {
            let index = row*cols+col;
            cipher.push(chars[index]);
        }
    }

    cipher
}

fn columnar_decypher (k1: &str, cipher_text: &str) -> String {
    let cols = k1.chars().count();

    assert!(cols > 0, "A key needs to be introduced.");

    let chars: Vec<char> = cipher_text.chars().collect();
    let rows = chars.len().div_ceil(cols);

    let order = col_alph_order(k1);
    let mut grid = vec!['X'; rows * cols];

    let mut index = 0;

    for &col in &order {
        for row in 0..rows {
            let position = row * cols + col;
            grid[position] = chars[index];
            index += 1;
        }
    }

    grid.into_iter().collect()
}

fn double_columnar_cipher (k1:&str, k2:&str, text:&str) -> String {
    let first = columnar_cypher(k1, text);
    columnar_cypher(k2,&first)
}

fn double_columnar_decypher (k1:&str, k2:&str, text:&str) -> String {
    let first = columnar_decypher(k2, text);
    columnar_decypher(k1, &first)
}

fn caesar_cypher (key: u8, text: &str) -> String {
    let mut cypher = String::new();

    for ch in text.chars() {
        if ch.is_ascii_uppercase() {
            let encrypt = ((ch as u8 + key - b'A') %26) + b'A';
            cypher.push(encrypt as char);
        } else if ch.is_ascii_lowercase() {
            let encrypt = ((ch as u8 + key - b'a') %26) + b'a';
            cypher.push(encrypt as char);
        } else {
            cypher.push(ch);
        }
    }

    cypher
}

fn caesar_decypher (key: u8, text: &str) -> String {
    let reverse_key = 26 - (key % 26);
    caesar_cypher(reverse_key, text)
}

fn crypto_bronium (k1: &str, k2: &str, k3: u8, text: &str) -> String {
    let crypto = double_columnar_cipher(k1, k2, text);
    caesar_cypher(k3, &crypto)
}

pub fn decrypto_bronium (k1: &str, k2: &str, k3: u8, text: &str) -> String {
    let caesar_plain = caesar_decypher(k3, text);
    double_columnar_decypher(k1, k2, &caesar_plain)
}

pub fn demo_4_1(text: &str) {
    println!("\n===========EXERCISE 4.1 COMMUNICATION DEMO===========");

    let alice = "ALice";
    let bob = "Bob";

    let k1 = "MANGO";
    let k2 = "TEBAS";
    let k3 = 3;

    println!("Sender: {}", alice);
    println!("Reciever {}", bob);

    println!("Keyes shared:");
    println!("Transposition key 1: {}", k1);
    println!("Transposition key 2: {}", k2);
    println!("Caesar key: {}", k3);

    println!("Original message:");
    println!("{}",text);

    println!("Alice encrypts the message using the CryptoBroniumTM software.");
    let encrypted = crypto_bronium(k1, k2, k3, text);

    println!("The encrypted message is sent.");
    println!("{}", encrypted);

    println!("Bob recieves the message and decrypts it with CryptoBroniumTM software.");
    let decrypted = decrypto_bronium(k1, k2, k3, &encrypted);

    println!("Decrypted message:");
    println!("{}", decrypted);

    println!("Successful CryptoBroniumTM encryption-decryption demo!");

}