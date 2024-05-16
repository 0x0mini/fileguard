use std::fs::File;
use std::io::{Read, Write};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use sha2::{Digest, Sha256};
use hex::{encode, decode};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const ENV_KEY: &str = "ENCRYPTION_KEY";

fn generate_iv() -> Vec<u8> {
    (0..16).map(|_| rand::random::<u8>()).collect()
}

fn hash_key(passphrase: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(passphrase.as_bytes());
    hasher.finalize().to_vec()
}

pub fn encrypt(data: &[u8], passphrase: &str) -> Result<Vec<u8>, &'static str> {
    let iv = generate_iv();
    let key = hash_key(passphrase);

    let cipher = Aes256Cbc::new_from_slices(&key, &iv).map_err(|_| "Error creating cipher")?;
    let mut buffer = iv; 
    buffer.extend_from_slice(&cipher.encrypt_vec(data));
    Ok(buffer)
}

pub fn decrypt(encrypted_data: &[u8], passphrase: &str) -> Result<Vec<u8>, &'static str> {
    if encrypted_data.len() < 16 {
        return Err("Encrypted data is too short");
    }

    let iv = &encrypted_data[..16]; 
    let data = &encrypted_data[16..];
    let key = hash_key(passphrase);

    let cipher = Aes256Cbc::new_from_slices(&key, iv).map_err(|_| "Error creating cipher")?;
    cipher.decrypt_vec(data).map_err(|_| "Decryption failed")
}

pub fn encrypt_file(file_path: &str, output_path: &str, passphrase: &str) -> Result<(), &'static str> {
    let mut file = File::open(file_path).map_err(|_| "Failed to open file")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).map_err(|_| "Failed to read file")?;

    let encrypted_data = encrypt(&data, passphrase)?;
    let mut output_file = File::create(output_path).map_err(|_| "Failed to create output file")?;
    output_file.write_all(&encrypted_data).map_err(|_| "Failed to write to output file")?;

    Ok(())
}

pub fn decrypt_file(encrypted_file_path: &str, output_path: &str, passphrase: &str) -> Result<(), &'static str> {
    let mut file = File::open(encrypted_file_path).map_err(|_| "Failed to open file")?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data).map_err(|_| "Failed to read file")?;

    let decrypted_data = decrypt(&encrypted_data, passphrase)?;
    let mut output_file = File::create(output_path).map_err(|_| "Failed to create output file")?;
    output_file.write_all(&decrypted_data).map_err(|_| "Failed to write to output file")?;

    Ok(())
}

fn main() {
    let passphrase = std::env::var(ENV_KEY).expect("ENCRYPTION_KEY not set in .env");
    if let Err(e) = encrypt_file("example.txt", "example.txt.enc", &passphrase) {
        eprintln!("Error encrypting file: {}", e);
    }
    
    if let Err(e) = decrypt_file("example.txt.enc", "example_decrypted.txt", &passphrase) {
        eprintln!("Error decrypting file: {}", e);
    }
}