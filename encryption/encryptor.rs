use std::fs::File;
use std::io::{Read, Write};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use sha2::{Digest, Sha256};
use hex::{encode, decode};

type Aes256CbcEncryption = Cbc<Aes256, Pkcs7>;

const ENCRYPTION_KEY_ENV_VARIABLE: &str = "ENCRYPTION_KEY";

fn create_initialization_vector() -> Vec<u8> {
    (0..16).map(|_| rand::random::<u8>()).collect()
}

fn derive_encryption_key(key_passphrase: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(key_passphrase.as_bytes());
    hasher.finalize().to_vec()
}

pub fn encrypt_bytes(input_bytes: &[u8], passphrase: &str) -> Result<Vec<u8>, &'static str> {
    let initialization_vector = create_initialization_vector();
    let encryption_key = derive_encryption_key(passphrase);

    let cipher = Aes256CbcEncryption::new_from_slices(&encryption_key, &initialization_vector)
                    .map_err(|_| "Error creating cipher")?;
    let mut encrypted_bytes = initialization_vector;
    encrypted_bytes.extend_from_slice(&cipher.encrypt_vec(input_bytes));
    Ok(encrypted_bytes)
}

pub fn decrypt_bytes(encrypted_bytes: &[u8], passphrase: &str) -> Result<Vec<u8>, &'static str> {
    if encrypted_bytes.len() < 16 {
        return Err("Encrypted bytes are too short");
    }

    let initialization_vector = &encrypted_bytes[..16];
    let encrypted_data_section = &encrypted_bytes[16..];
    let encryption_key = derive_encryption_key(passphrase);

    let cipher = Aes256CbcEncryption::new_from_slices(&encryption_key, initialization_vector)
                    .map_err(|_| "Error creating cipher")?;
    cipher.decrypt_vec(encrypted_data_section).map_err(|_| "Decryption failed")
}

pub fn encrypt_file_to_path(source_file_path: &str, encrypted_file_path: &str, passphrase: &str) -> Result<(), &'static str> {
    let mut source_file = File::open(source_file_path).map_err(|_| "Failed to open file")?;
    let mut source_contents = Vec::new();
    source_file.read_to_end(&mut source_contents).map_err(|_| "Failed to read file")?;

    let encrypted_data = encrypt_bytes(&source_contents, passphrase)?;
    let mut destination_file = File::create(encrypted_file_path).map_err(|_| "Failed to create output file")?;
    destination_file.write_all(&encrypted_data).map_err(|_| "Failed to write to output file")?;

    Ok(())
}

pub fn decrypt_file_to_path(encrypted_source_file_path: &str, decrypted_file_path: &str, passphrase: &str) -> Result<(), &'static str> {
    let mut encrypted_source_file = File::open(encrypted_source_file_path).map_err(|_| "Failed to open file")?;
    let mut encrypted_file_contents = Vec::new();
    encrypted_source_file.read_to_end(&mut encrypted_file_contents).map_err(|_| "Failed to read file")?;

    let decrypted_data = decrypt_bytes(&encrypted_file_contents, passphrase)?;
    let mut decrypted_file = File::create(decrypted_file_path).map_err(|_| "Failed to create output file")?;
    decrypted_file.write_all(&decrypted_data).map_err(|_| "Failed to write to output file")?;

    Ok(())
}

fn main() {
    let key_passphrase = std::env::var(ENCRYPTION_KEY_ENV_VARIABLE).expect("ENCRYPTION_KEY not set in .env");
    if let Err(encrypt_error) = encrypt_file_to_path("example.txt", "example.txt.enc", &key_passphrase) {
        eprintln!("Error encrypting file: {}", encrypt_error);
    }
    
    if let Err(decrypt_error) = decrypt_file_to_path("example.txt.enc", "example_decrypted.txt", &key_passphrase) {
        eprintln!("Error decrypting file: {}", decrypt_error);
    }
}