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

fn double_columnar_cipher (k1:&str, k2:&str, text:&str) -> String {
    let first = columnar_cypher(k1, text);
    columnar_cypher(k2,&first)
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

pub fn crypto_bronium (k1: &str, k2: &str, k3: u8, text: &str) -> String {
    let crypto = double_columnar_cipher(k1, k2, text);
    caesar_cypher(k3, &crypto)
}