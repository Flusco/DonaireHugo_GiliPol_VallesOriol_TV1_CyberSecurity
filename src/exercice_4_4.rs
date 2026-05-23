use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};

const HASH_FILE: &str = "hashes_4_4.txt";

fn assignment_files() -> Vec<&'static str> {
    vec![
        "main.rs",
        "menu.rs",
        "exercice_4_1.rs",
        "exercice_4_2.rs",
        "exercice_4_3.rs",
        "exercice_4_4.rs",
        "exercice_4_5.rs",
        "Cargo.toml",
    ]
}

fn find_file(file_name: &str) -> Option<String> {
    let possible_paths = vec![file_name.to_string(), format!("src/{}", file_name)];

    for path in possible_paths {
        if fs::metadata(&path).is_ok() {
            return Some(path);
        }
    }

    None
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn save_hashes() -> io::Result<()> {
    let mut output = File::create(HASH_FILE)?;

    println!("\nGenerating SHA-256 hashes...\n");

    for file_name in assignment_files() {
        match find_file(file_name) {
            Some(path) => {
                let hash = hash_file(&path)?;
                println!("{}  {}", hash, path);
                writeln!(output, "{}  {}", hash, path)?;
            }
            None => {
                println!("Missing file, skipped: {}", file_name);
            }
        }
    }

    println!("\nHashes saved in {}", HASH_FILE);
    Ok(())
}

fn check_hashes() -> io::Result<()> {
    let text = fs::read_to_string(HASH_FILE)?;
    let mut correct = 0;
    let mut changed = 0;
    let mut missing = 0;

    println!("\nChecking files with {}...\n", HASH_FILE);

    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let old_hash = match parts.next() {
            Some(value) => value,
            None => continue,
        };
        let path = match parts.next() {
            Some(value) => value,
            None => continue,
        };

        match hash_file(path) {
            Ok(new_hash) => {
                if new_hash == old_hash {
                    println!("OK: {}", path);
                    correct += 1;
                } else {
                    println!("MODIFIED: {}", path);
                    changed += 1;
                }
            }
            Err(_) => {
                println!("MISSING: {}", path);
                missing += 1;
            }
        }
    }

    println!("\nSummary: {} OK, {} modified, {} missing", correct, changed, missing);
    Ok(())
}

pub fn generate_hashes() {
    if let Err(error) = save_hashes() {
        println!("Error generating hashes: {}", error);
    }
}

pub fn verify_integrity() {
    if let Err(error) = check_hashes() {
        println!("Error checking hashes: {}", error);
        println!("Generate the hashes first with option 1.");
    }
}
