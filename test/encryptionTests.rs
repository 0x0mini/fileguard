#[cfg(test)]
mod tests {
    use super::*; 
    use std::env;

    fn setup() {
        env::set_var("ENCRYPTION_KEY", "secret_key");
    }

    #[test]
    fn test_encrypt_decrypt() {
        setup();

        let encryption_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not found");

        let original_message = "RustEncryptionTest";
        let encrypted_message = encrypt(original_message, &encryption_key).expect("Encryption failed");
        let decrypted_message = decrypt(&encrypted_message, &encryption_key).expect("Decryption failed");

        assert_eq!(original_message, decrypted_message, "Decrypted text did not match the original message");
    }

    #[test]
    fn test_encrypt_different_keys() {
        setup();

        let correct_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not found");
        let wrong_key = "incorrect_key";

        let message = "AnotherTestMessage";
        let encrypted_message = encrypt(message, &correct_key).expect("Encryption failed");

        match decrypt(&encrypted_message, &wrong_key) {
            Ok(decrypted_message) => assert_ne!(message, decrypted_message, "Decryption succeeded with the wrong key"),
            Err(_) => assert!(true, "Encryption failed as expected with an incorrect key"),
        }
    }

    #[test]
    fn test_empty_string_encryption() {
        setup();

        let encryption_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not found");

        let original_message = "";
        let encrypted_message = encrypt(original_message, &encryption_key).expect("Encryption of empty string failed");
        let decrypted_message = decrypt(&encrypted_message, &encryption_key).expect("Decryption of empty string failed");

        assert_eq!(original_message, decrypted_message, "Decrypted empty string did not match the original message");
    }
}