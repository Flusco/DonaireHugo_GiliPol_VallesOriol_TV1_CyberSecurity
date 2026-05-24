/* 
    Computer Security TV1 - Library for Menus
    Author: Hugo Donaire, Oriol Vallès, Pol Gili
*/

use std::io;
use crate::exercice_4_3;
use crate::exercice_4_2;
use crate::exercice_4_1;
use crate::exercice_4_4;
use crate::exercice_4_6;

fn visuals_main_menu() {
    println!("\n================TV1 CyberSecurity================");
    println!("1. Exercise 4.1 - Classical Encryption and Custom Cipher ");
    println!("2. Exercise 4.2 - Symmetric Encryption");
    println!("3. Exercise 4.3 - Asymmetric Encryption and Secure Communication");
    println!("4. Exercise 4.4 - Hashing and Integrity");
    println!("5. Exercise 4.5 - Password Cracking");
    println!("6. Exercise 4.6 - Cryptanalysis");
    println!("7. Exit");
    println!("Choose an option:");
}

fn visuals_exercise_4_2_menu() {
    println!("\n================ EXERCISE 4.2 ================");
    println!("1. Run symmetric encryption demo");
    println!("2. Back to main menu");
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
    println!("2. Back to main menu");
    println!("Choose an option:");
}

fn visuals_exercise_4_4_menu() {
    println!("\n================ EXERCISE 4.4 ================");
    println!("1. Generate SHA-256 hashes");
    println!("2. Check integrity with saved hashes");
    println!("3. Back to main menu");
    println!("Choose an option:");
}

fn visuals_exercise_4_6_menu() {
    println!("\n================ EXERCISE 4.6 ================");
    println!("1. Analyze Exercise 4.1 custom cipher");
    println!("2. Analyze Exercise 4.3 secure communication");
    println!("3. Run both analyses");
    println!("4. Back to main menu");
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
                println!("Incorrect option: Choose 1 or 2")
            }
        }
    }
}

fn exercice_4_2_menu() {
    loop {
        visuals_exercise_4_2_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");

        match option.trim() {
            "1" => {
                exercice_4_2::encription();
            }
            "2" => {
                break;
            }
            _ => {
                println!("Incorrect option, choose: 1 or 2")
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
                    println!("Message too long to encrypt! Please write a shorter one.");
                    continue;
                }

                exercice_4_3::communication_demo(message.as_bytes());
            }

            "2" => {
                let message = read_message_user();

                if message.as_bytes().len() > 180 {
                    println!("Message too long to encrypt! Please write a shorter one.");
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

fn exercice_4_4_menu() {
    loop {
        visuals_exercise_4_4_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");

        match option.trim() {
            "1" => {
                exercice_4_4::generate_hashes();
            }
            "2" => {
                exercice_4_4::verify_integrity();
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

fn exercice_4_6_menu() {
    loop {
        visuals_exercise_4_6_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");

        match option.trim() {
            "1" => {
                exercice_4_6::cryptanalysis_4_1();
            }
            "2" => {
                exercice_4_6::cryptanalysis_4_3();
            }
            "3" => {
                exercice_4_6::full_cryptanalysis();
            }
            "4" => {
                break;
            }
            _ => {
                println!("Incorrect option, choose: 1, 2, 3 or 4")
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
            "2" => {
                exercice_4_2_menu();
            }
            "3" => {
                exercice_4_3_menu();
            }
            "4" => {
                exercice_4_4_menu();
            }
            "6" => {
                exercice_4_6_menu();
            }
            "7" => {
                println!("Closing program...");
                break;
            }
            _ => {
                println!("Incorrect option, choose: 1, 2, 3, 4, 6 or 7");
            }
        }
    }
}

fn read_message_user() -> String {
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
