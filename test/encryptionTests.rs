#[cfg(test)]
mod tests {
    use super::*; 
    use std::env;

    fn setup_test_environment() {
        env::set_var("ENCRYPTION_KEY", "secret_key");
    }

    #[test]
    fn encryption_and_decryption_with_same_key() {
        setup_test_environment();

        let encryption_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not found");

        let original_text = "RustEncryptionTest";
        let encrypted_text = encrypt(original_text, &encryption_key).expect("Encryption failed");
        let decrypted_text = decrypt(&encrypted_text, &encryption_key).expect("Decryption failed");

        assert_eq!(original_text, decrypted_text, "Decrypted text did not match the original text");
    }

    #[test]
    fn decryption_fails_with_incorrect_key() {
        setup_test_environment();

        let valid_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not found");
        let invalid_key = "incorrect_key";

        let sample_message = "AnotherTestMessage";
        let encrypted_message = encrypt(sample_message, &valid_key).expect("Encryption failed");

        match decrypt(&encrypted_message, &invalid_key) {
            Ok(decrypted_with_wrong_key) => assert_ne!(sample_message, decrypted_with_wrong_key, "Decryption should not succeed with the wrong key"),
            Err(_) => assert!(true, "Failed decryption with an incorrect key as expected"),
        }
    }

    #[test]
    fn encryption_and_decryption_of_empty_string() {
        setup_test_environment();

        let encryption_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not found");

        let empty_message = "";
        let encrypted_empty_message = encrypt(empty_message, &encryption_key).expect("Failed to encrypt empty string");
        let decrypted_empty_message = decrypt(&encrypted_empty_message, &encryption_key).expect("Failed to decrypt empty string");

        assert_eq!(empty_message, decrypted_empty_message, "Decrypted empty string did not match the original empty message");
    }
}