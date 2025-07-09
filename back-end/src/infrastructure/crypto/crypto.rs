use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng as OsRngAes, generic_array::{GenericArray}}
};
use hex::{decode, encode};


const KEY_LENGTH: usize = 128; // 256 bits


pub fn derive_password_hash(password: String, salt: Vec<u8>) -> String {
    let mut password_hash = vec![0u8; KEY_LENGTH];

    let n = 10000;

    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, n, &mut password_hash);

    encode(password_hash)
}

pub fn encrypt(key: &str, message: String) -> Result<(String, String), String> {
    if key.as_bytes().len() != 32 {
        return Err("Chave inválida: deve conter exatamente 32 bytes para AES-256".to_string());
    }
    
    let key_array = GenericArray::clone_from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(&key_array);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRngAes); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, message.as_bytes()).map_err(|e| format!("Erro ao criptografar: {:?}", e))?;
    
    let cipher_text_result = encode(ciphertext);
    let nonce_result = encode(nonce.to_vec());
    Ok((cipher_text_result, nonce_result))
}

pub fn decrypt(key: &str, data: String, nonce: String) -> Result<String, String> {
    if key.as_bytes().len() != 32 {
        return Err("Chave inválida: deve conter exatamente 32 bytes para AES-256".to_string());
    }

    let key_array = GenericArray::clone_from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(&key_array);

    let ciphertext = decode(&data).map_err(|e| format!("Base64 ciphertext inválido: {}", e))?;
    let nonce_bytes = decode(&nonce).map_err(|e| format!("Base64 nonce inválido: {}", e))?;

    if nonce_bytes.len() != 12 {
        return Err("Nonce inválido: deve conter exatamente 12 bytes".to_string());
    }

    let nonce_array = GenericArray::clone_from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(&nonce_array, ciphertext.as_ref())
        .map_err(|e| format!("Erro ao descriptografar: {:?}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 inválido: {:?}", e))
}


#[cfg(test)]
mod tests {
    use super::*;

    use rand::rngs::OsRng;
    use rand::{TryRngCore};

    #[test]
    fn test_encrypt_decrypt_sucess() {
        let key_str_1 = "0123456789abcdefABCDEFghijklmnop";

        let key_str_2 = "key-for-tests-number-03-********";

        let plaintext_1 = "testando@testando.com".to_string();
        let plaintext_2 = "98765".to_string();
        let plaintext_3 = "Kelly Grayson".to_string();
        let plaintext_4 = "09/09/1999".to_string();

        let (encrypted_1_1, nonce_1_1) = encrypt(&key_str_1, plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_1_1 = decrypt(&key_str_1, encrypted_1_1, nonce_1_1).expect("decrypt 1 falhou");

        let (encrypted_1_2, nonce_1_2) = encrypt(&key_str_1, plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_1_2 = decrypt(&key_str_1, encrypted_1_2, nonce_1_2).expect("decrypt 2 falhou");

        let (encrypted_1_3, nonce_1_3) = encrypt(&key_str_1, plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_1_3 = decrypt(&key_str_1, encrypted_1_3, nonce_1_3).expect("decrypt 3 falhou");

        let (encrypted_1_4, nonce_1_4) = encrypt(&key_str_1, plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_1_4 = decrypt(&key_str_1, encrypted_1_4, nonce_1_4).expect("decrypt 4 falhou");


        let (encrypted_2_1, nonce_2_1) = encrypt(&key_str_2, plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_2_1 = decrypt(&key_str_2, encrypted_2_1, nonce_2_1).expect("decrypt 1 falhou");

        let (encrypted_2_2, nonce_2_2) = encrypt(&key_str_2, plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_2_2 = decrypt(&key_str_2, encrypted_2_2, nonce_2_2).expect("decrypt 2 falhou");

        let (encrypted_2_3, nonce_2_3) = encrypt(&key_str_2, plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_2_3 = decrypt(&key_str_2, encrypted_2_3, nonce_2_3).expect("decrypt 3 falhou");

        let (encrypted_2_4, nonce_2_4) = encrypt(&key_str_2, plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_2_4 = decrypt(&key_str_2, encrypted_2_4, nonce_2_4).expect("decrypt 4 falhou");

        assert_eq!(decrypted_1_1, decrypted_2_1);
        assert_eq!(decrypted_1_2, decrypted_2_2);
        assert_eq!(decrypted_1_3, decrypted_2_3);
        assert_eq!(decrypted_1_4, decrypted_2_4);
    }

    #[test]
    fn test_encrypt_decrypt_failure_keys() {
        let key_str_1 = "supersecretkeyvaluefortestcase!!";
        let key_str_2 = "supersecretkeyvaluefortestcase@@";

        let plaintext_1 = "mensagem sensível".to_string();
        let plaintext_2 = "12345".to_string();
        let plaintext_3 = "Ed Mercer".to_string();
        let plaintext_4 = "01/01/2000".to_string();
        let plaintext_5 = "teste@teste.com".to_string();

        let (encrypted_1, nonce_1) = encrypt(&key_str_1, plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_1 = decrypt(&key_str_2, encrypted_1, nonce_1);

        let (encrypted_2, nonce_2) = encrypt(&key_str_1, plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_2 = decrypt(&key_str_2, encrypted_2, nonce_2);

        let (encrypted_3, nonce_3) = encrypt(&key_str_1, plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_3 = decrypt(&key_str_2, encrypted_3, nonce_3);

        let (encrypted_4, nonce_4) = encrypt(&key_str_1, plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_4 = decrypt(&key_str_2, encrypted_4, nonce_4);

        let (encrypted_5, nonce_5) = encrypt(&key_str_1, plaintext_5.clone()).expect("encrypt 5 falhou");
        let decrypted_5 = decrypt(&key_str_2, encrypted_5, nonce_5);

        assert!(decrypted_1.is_err());
        assert!(decrypted_2.is_err());
        assert!(decrypted_3.is_err());
        assert!(decrypted_4.is_err());
        assert!(decrypted_5.is_err());
    }

    #[test]
    fn test_encrypt_decrypt_failure_plaintext() {
        let key_str = "abc123ABC!@#xyz789XYZ$%^lmn456LM";

        let plaintext_1 = "teste@teste.com".to_string();
        let plaintext_2 = "98765".to_string();
        let plaintext_3 = "Kelly Grayson".to_string();
        let plaintext_4 = "09/09/1999".to_string();

        let (encrypted_1, nonce_1) = encrypt(&key_str, plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_1 = decrypt(&key_str, encrypted_1, nonce_1).expect("decrypt 1 falhou");

        let (encrypted_2, nonce_2) = encrypt(&key_str, plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_2 = decrypt(&key_str, encrypted_2, nonce_2).expect("decrypt 2 falhou");

        let (encrypted_3, nonce_3) = encrypt(&key_str, plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_3 = decrypt(&key_str, encrypted_3, nonce_3).expect("decrypt 3 falhou");

        let (encrypted_4, nonce_4) = encrypt(&key_str, plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_4 = decrypt(&key_str, encrypted_4, nonce_4).expect("decrypt 4 falhou");

        assert_ne!(plaintext_1, decrypted_2);
        assert_ne!(plaintext_3, decrypted_4);
        assert_ne!(plaintext_2, decrypted_3);
        assert_ne!(plaintext_4, decrypted_1);
    }

    #[test]
    fn test_encrypt_decrypt_failure_nonce() {
        let key_str = "12345678901234567890123456789012";

        let plaintext_1 = "teste@teste.com".to_string();
        let plaintext_2 = "98765".to_string();
        let plaintext_3 = "Kelly Grayson".to_string();
        let plaintext_4 = "09/09/1999".to_string();

        let (encrypted_1, nonce_1) = encrypt(&key_str, plaintext_1.clone()).expect("encrypt 1 falhou");
        let (encrypted_2, nonce_2) = encrypt(&key_str, plaintext_2.clone()).expect("encrypt 2 falhou");
        let (encrypted_3, nonce_3) = encrypt(&key_str, plaintext_3.clone()).expect("encrypt 3 falhou");
        let (encrypted_4, nonce_4) = encrypt(&key_str, plaintext_4.clone()).expect("encrypt 4 falhou");

        let decrypted_1 = decrypt(&key_str, encrypted_1, nonce_4);
        let decrypted_2 = decrypt(&key_str, encrypted_2, nonce_1);
        let decrypted_3 = decrypt(&key_str, encrypted_3, nonce_2);
        let decrypted_4 = decrypt(&key_str, encrypted_4, nonce_3);

        assert!(decrypted_1.is_err());
        assert!(decrypted_2.is_err());
        assert!(decrypted_3.is_err());
        assert!(decrypted_4.is_err());
    }

    #[test]
    fn test_derive_password_hash_sucess() {
        let mut salt = vec![0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut salt);

        let password_1 = "12345";
        let password_2 = "a1b2c3";
        let password_3 = "senha";
        let password_4 = "segredo";
        let password_5 = "b&a0TcR!";

        let password_1_hash_1 = derive_password_hash(password_1.to_string(), salt.clone());
        let password_1_hash_2 = derive_password_hash(password_2.to_string(), salt.clone());
        let password_1_hash_3 = derive_password_hash(password_3.to_string(), salt.clone());
        let password_1_hash_4 = derive_password_hash(password_4.to_string(), salt.clone());
        let password_1_hash_5 = derive_password_hash(password_5.to_string(), salt.clone());

        let password_2_hash_1 = derive_password_hash(password_1.to_string(), salt.clone());
        let password_2_hash_2 = derive_password_hash(password_2.to_string(), salt.clone());
        let password_2_hash_3 = derive_password_hash(password_3.to_string(), salt.clone());
        let password_2_hash_4 = derive_password_hash(password_4.to_string(), salt.clone());
        let password_2_hash_5 = derive_password_hash(password_5.to_string(), salt.clone());

        assert_eq!(password_1_hash_1, password_2_hash_1);
        assert_eq!(password_1_hash_2, password_2_hash_2);
        assert_eq!(password_1_hash_3, password_2_hash_3);
        assert_eq!(password_1_hash_4, password_2_hash_4);
        assert_eq!(password_1_hash_5, password_2_hash_5);
    }

    #[test]
    fn test_derive_password_hash_failure_salts() {
        let mut salt_1 = vec![0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut salt_1);

        let mut salt_2 = vec![0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut salt_2);

        let password_1 = "12345";
        let password_2 = "a1b2c3";
        let password_3 = "senha";
        let password_4 = "segredo";
        let password_5 = "b&a0TcR!";

        let password_1_hash_1 = derive_password_hash(password_1.to_string(), salt_1.clone());
        let password_1_hash_2 = derive_password_hash(password_2.to_string(), salt_1.clone());
        let password_1_hash_3 = derive_password_hash(password_3.to_string(), salt_1.clone());
        let password_1_hash_4 = derive_password_hash(password_4.to_string(), salt_1.clone());
        let password_1_hash_5 = derive_password_hash(password_5.to_string(), salt_1.clone());

        let password_2_hash_1 = derive_password_hash(password_1.to_string(), salt_2.clone());
        let password_2_hash_2 = derive_password_hash(password_2.to_string(), salt_2.clone());
        let password_2_hash_3 = derive_password_hash(password_3.to_string(), salt_2.clone());
        let password_2_hash_4 = derive_password_hash(password_4.to_string(), salt_2.clone());
        let password_2_hash_5 = derive_password_hash(password_5.to_string(), salt_2.clone());

        assert_ne!(password_1_hash_1, password_2_hash_1);
        assert_ne!(password_1_hash_2, password_2_hash_2);
        assert_ne!(password_1_hash_3, password_2_hash_3);
        assert_ne!(password_1_hash_4, password_2_hash_4);
        assert_ne!(password_1_hash_5, password_2_hash_5);
    }

    #[test]
    fn test_derive_password_hash_failure_password() {
        let mut salt = vec![0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut salt);

        let password_1 = "12345";
        let password_2 = "a1b2c3";
        let password_3 = "senha";
        let password_4 = "segredo";

        let password_hash_1 = derive_password_hash(password_1.to_string(), salt.clone());
        let password_hash_2 = derive_password_hash(password_2.to_string(), salt.clone());
        let password_hash_3 = derive_password_hash(password_3.to_string(), salt.clone());
        let password_hash_4 = derive_password_hash(password_4.to_string(), salt.clone());

        assert_ne!(password_hash_1, password_hash_2);
        assert_ne!(password_hash_3, password_hash_4);
        assert_ne!(password_hash_1, password_hash_4);
        assert_ne!(password_hash_3, password_hash_2);
    }
}
