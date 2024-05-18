use std::fs::File;
use std::io::{Read, Write};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use sha2::{Digest, Sha256};
use hex::{encode, decode};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const ENCRYPTION_KEY_ENV_VAR: &str = "ENCRYPTION_KEY";

fn generate_initialization_vector() -> Vec<u8> {
    (0..16).map(|_| rand::random::<u8>()).collect()
}

fn hash_encryption_key(passphrase: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(passphrase.as_bytes());
    hasher.finalize().to_vec()
}

pub fn encrypt_data(data: &[u8], passphrase: &str) -> Result<Vec<u8>, &'static str> {
    let iv = generate_initialization_vector();
    let key = hash_encryption_key(passphrase);

    let cipher = Aes256Cbc::new_from_slices(&key, &iv).map_err(|_| "Error creating cipher")?;
    let mut encrypted_data = iv;
    encrypted_data.extend_from_slice(&cipher.encrypt_vec(data));
    Ok(encrypted_data)
}

pub fn decrypt_data(encrypted_data: &[u8], passphrase: &str) -> Result<Vec<u8>, &'static str> {
    if encrypted_data.len() < 16 {
        return Err("Encrypted data is too short");
    }

    let iv = &encrypted_data[..16];
    let data = &encrypted_data[16..];
    let key = hash_encryption_key(passphrase);

    let cipher = Aes256Cbc::new_from_slices(&key, iv).map_err(|_| "Error creating cipher")?;
    cipher.decrypt_vec(data).map_err(|_| "Decryption failed")
}

pub fn encrypt_file(file_path: &str, output_file_path: &str, passphrase: &str) -> Result<(), &'static str> {
    let mut input_file = File::open(file_path).map_err(|_| "Failed to open file")?;
    let mut file_contents = Vec::new();
    input_file.read_to_end(&mut file_contents).map_err(|_| "Failed to read file")?;

    let encrypted_contents = encrypt_data(&file_contents, passphrase)?;
    let mut output_file = File::create(output_file_path).map_err(|_| "Failed to create output file")?;
    output_file.write_all(&encrypted_contents).map_err(|_| "Failed to write to output file")?;

    Ok(())
}

pub fn decrypt_file(encrypted_file_path: &str, decrypted_file_path: &str, passphrase: &str) -> Result<(), &'static str> {
    let mut encrypted_file = File::open(encrypted_file_path).map_err(|_| "Failed to open file")?;
    let mut encrypted_contents = Vec::new();
    encrypted_file.read_to_end(&mut encrypted_contents).map_err(|_| "Failed to read file")?;

    let decrypted_contents = decrypt_data(&encrypted_contents, passphrase)?;
    let mut output_file = File::create(decrypted_file_path).map_err(|_| "Failed to create output file")?;
    output_file.write_all(&decrypted_contents).map_err(|_| "Failed to write to output file")?;

    Ok(())
}

fn main() {
    let passphrase = std::env::var(ENCRYPTION_KEY_ENV_VAR).expect("ENCRYPTION_KEY not set in .env");
    if let Err(e) = encrypt_file("example.txt", "example.txt.enc", &passphrase) {
        eprintln!("Error encrypting file: {}", e);
    }
    
    if let Err(e) = decrypt_file("example.txt.enc", "example_decrypted.txt", &passphrase) {
        eprintln!("Error decrypting file: {}", e);
    }
}