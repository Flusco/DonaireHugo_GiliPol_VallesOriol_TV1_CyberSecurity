/* 
    Computer Security TV1 - Library for Menus
    Author: Hugo Donaire, Oriol Vallès, Pol Gili
*/

use std::io;
use crate::exercice_4_3;
use crate::exercice_4_1;

fn visuals_main_menu() {
    println!("\n================TV1 CyberSecurity================");
    println!("1. Exercise 4.1 - Classical Encryption and Custom Cipher ");
    println!("2. Exercise 4.2 - Symmetric Encryption");
    println!("3. Exercise 4.3 - Asymmetric Encryption and Secure Communication");
    println!("4. Exercise 4.4 - Hashing and Integrity");
    println!("5. Exercise 4.5 - Password Cracking");
    println!("6. Exercise 4.6 -  Cryptanalysis");

    println!("2. Exit");
    println!("Choose an option:");
}

fn visuals_exercise_4_3_menu() {
    println!("\n================ EXERCISE 4.3 ================");
    println!("1. Secure communication demo");
    println!("2. Man-in-the-Middle attack demo");
    println!("3. Back to main menu");
    println!("Choose an option:");
}

fn visuals_exercise_4_1_menu() {
    println!("\n================ EXERCISE 4.1 ================");
    println!("1. Secure communication demo");
    println!("2. Exit");
    println!("Choose an option:");
}

fn exercice_4_1_menu() {
    loop {
        visuals_exercise_4_1_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");

        match option.trim() {
            "1" => {
                let text = &read_message_user();
                exercice_4_1::demo_4_1(text);
            }
            "2" => {
                break;
            }
            _ => {
                println!("Incorrect option: CHoose 1!")
            }
        }
    }
}

fn exercice_4_3_menu() {
    loop {
        visuals_exercise_4_3_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");

        match option.trim() {
            "1" => {
                let message = read_message_user();

                if message.as_bytes().len() > 180 {
                    println!("Messafe to long to encrypt! Please write a shorter one.");
                    continue;
                }

                exercice_4_3::communication_demo(message.as_bytes());
            }

            "2" => {
                let message = read_message_user();

                if message.as_bytes().len() > 180 {
                    println!("Messafe to long to encrypt! Please write a shorter one.");
                    continue;
                }

                exercice_4_3::mitm_attack_demo(message.as_bytes());
            }

            "3" => {
                break;
            }

            _ => {
                println!("Incorrect option, choose: 1, 2 or 3")
            }
        }
    }
}

pub fn main_menu() {
    loop {
        visuals_main_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");

        match option.trim() {
            "1" => {
                exercice_4_1_menu();
            }
            "3" => {
                exercice_4_3_menu();
            }
            "4" => {
                println!("Closing program...");
                break;
            }
            _ => {
                println!("Incorrect option, choose:");
            }
        }
    }
}

fn read_message_user () -> String {
    println!("\nWrite the text to encrypt:");
    
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read text");

    let message = message.trim().to_string();

    if message.is_empty() {
        String::from("Hello Eric, this is default text.")
    } else {
        message
    }
}