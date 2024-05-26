use aes_gcm::{aead::{Aead, NewAead}, Aes256Gcm, generic_array::GenericArray};
use hex;
use std::{
    env,
    fs::File,
    io::{self, Read, Write, ErrorKind},
    path::Path,
};

mod file_decryptor {
    use super::*;

    pub fn decrypt_file(file_path: &Path, key: &[u8], nonce: &[u8]) -> io::Result<()> {
        if !verify_permissions() {
            return Err(io::Error::new(
                ErrorKind::PermissionDenied,
                "Insufficient permissions.",
            ));
        }

        let mut file = File::open(file_path)?;
        let mut encrypted_contents = Vec::new();
        file.read_to_end(&mut encrypted_contents)?;

        let cipher = Aes256Gcm::new(GenericArray::from_slice(key));

        let decrypted_contents = cipher
            .decrypt(GenericArray::from_slice(nonce), encrypted_contents.as_ref())
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Decryption failed"))?;

        let mut file = File::create(file_path)?;
        file.write_all(&decrypted_contents)?;

        Ok(())
    }

    fn verify_permissions() -> bool {
        env::var("USER_PERMISSIONS").map_or(false, |val| val == "admin")
    }
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let file_to_decrypt = Path::new("encrypted_file.dat");

    let decryption_key = hex::decode(env::var("DECRYPTION_KEY").expect("DECRYPTION_KEY must be set"))
        .expect("Failed to decode DECRYPTION_KEY");
    let nonce = hex::decode(env::var("DECRYPTION_NONCE").expect("DECRYPTION_NONCE must be set"))
        .expect("Failed to decode DECRYPTION_NONCE");

    file_decryptor::decrypt_file(&file_to_decrypt, &decryption_key, &nonce).map_err(|e| {
        eprintln!("Error decrypting file: {}", e);
        e
    })?;

    println!("File successfully decrypted.");
    Ok(())
}