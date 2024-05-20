use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use aes_gcm::{Aes256Gcm, aead::{Aead, NewAead, generic_array::GenericArray}};
use hex;

mod file_decryptor {
    use super::*;

    pub fn decrypt_file(file_path: &Path, key: &[u8], nonce: &[u8]) -> io::Result<()> {
        if !verify_permissions() {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Insufficient permissions."));
        }

        let mut file = File::open(file_path)?;
        let mut encrypted_contents = vec![];
        file.read_to_end(&mut encrypted_contents)?;

        let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
        let nonce = GenericArray::from_slice(nonce);

        let decrypted_contents = cipher.decrypt(nonce, encrypted_contents.as_ref())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Decryption failed"))?;

        let mut file = File::create(file_path)?;
        file.write_all(&decrypted_contents)?;
        
        Ok(())
    }

    fn verify_permissions() -> bool {
        match env::var("USER_PERMISSIONS") {
            Ok(val) => val == "admin",
            Err(_) => false,
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let file_to_decrypt = Path::new("encrypted_file.dat");
    let decryption_key = hex::decode(env::var("DECRYPTION_KEY").expect("DECRYPTION_KEY must be set"))
        .expect("Failed to decode DECRYPTION_KEY");
    let nonce = hex::decode(env::var("DECRYPTION_NONCE").expect("DECRYPTION_NONCE must be set"))
        .expect("Failed to decode DECRYPTION_NONCE");

    match file_decryptor::decrypt_file(file_to_decrypt, &decryption_key, &nonce) {
        Ok(_) => println!("File successfully decrypted."),
        Err(e) => eprintln!("Error decrypting file: {}", e),
    }
}