use pbkdf2::pbkdf2_hmac;
use sha2::{Digest, Sha256};
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng as OsRngAes, generic_array::{GenericArray}}
};
use rand::{TryRngCore, rngs::OsRng};
use hex::{decode, encode};


const KEY_LENGTH: usize = 128; // 256 bits


pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 16];
    let _ = OsRng.try_fill_bytes(&mut salt);
    salt
}

pub fn simple_hash(key: &[u8], data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(key);

    encode(hasher.finalize().to_vec())
}

pub fn derive_password_hash(password: String, salt: Vec<u8>) -> Vec<u8> {
    let mut password_hash = vec![0u8; KEY_LENGTH];

    let n = 10000;

    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, n, &mut password_hash);

    // encode(password_hash)
    password_hash
}

pub fn encrypt(key: &[u8], message: String) -> Result<(String, String), String> {
    if key.len() != 32 {
        return Err("Chave inválida: deve conter exatamente 32 bytes para AES-256".to_string());
    }
    
    let key_array = GenericArray::clone_from_slice(&key);
    let cipher = Aes256Gcm::new(&key_array);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRngAes); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, message.as_bytes()).map_err(|e| format!("Erro ao criptografar: {:?}", e))?;
    
    let cipher_text_result = encode(ciphertext);
    let nonce_result = encode(nonce.to_vec());
    Ok((cipher_text_result, nonce_result))
}

pub fn decrypt(key: &[u8], data: String, nonce: String) -> Result<String, String> {
    if key.len() != 32 {
        return Err("Chave inválida: deve conter exatamente 32 bytes para AES-256".to_string());
    }

    let key_array = GenericArray::clone_from_slice(&key);
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

        let (encrypted_1_1, nonce_1_1) = encrypt(key_str_1.as_bytes(), plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_1_1 = decrypt(key_str_1.as_bytes(), encrypted_1_1, nonce_1_1).expect("decrypt 1 falhou");

        let (encrypted_1_2, nonce_1_2) = encrypt(key_str_1.as_bytes(), plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_1_2 = decrypt(key_str_1.as_bytes(), encrypted_1_2, nonce_1_2).expect("decrypt 2 falhou");

        let (encrypted_1_3, nonce_1_3) = encrypt(key_str_1.as_bytes(), plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_1_3 = decrypt(key_str_1.as_bytes(), encrypted_1_3, nonce_1_3).expect("decrypt 3 falhou");

        let (encrypted_1_4, nonce_1_4) = encrypt(key_str_1.as_bytes(), plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_1_4 = decrypt(key_str_1.as_bytes(), encrypted_1_4, nonce_1_4).expect("decrypt 4 falhou");


        let (encrypted_2_1, nonce_2_1) = encrypt(key_str_2.as_bytes(), plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_2_1 = decrypt(key_str_2.as_bytes(), encrypted_2_1, nonce_2_1).expect("decrypt 1 falhou");

        let (encrypted_2_2, nonce_2_2) = encrypt(key_str_2.as_bytes(), plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_2_2 = decrypt(key_str_2.as_bytes(), encrypted_2_2, nonce_2_2).expect("decrypt 2 falhou");

        let (encrypted_2_3, nonce_2_3) = encrypt(key_str_2.as_bytes(), plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_2_3 = decrypt(key_str_2.as_bytes(), encrypted_2_3, nonce_2_3).expect("decrypt 3 falhou");

        let (encrypted_2_4, nonce_2_4) = encrypt(key_str_2.as_bytes(), plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_2_4 = decrypt(key_str_2.as_bytes(), encrypted_2_4, nonce_2_4).expect("decrypt 4 falhou");

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

        let (encrypted_1, nonce_1) = encrypt(key_str_1.as_bytes(), plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_1 = decrypt(key_str_2.as_bytes(), encrypted_1, nonce_1);

        let (encrypted_2, nonce_2) = encrypt(key_str_1.as_bytes(), plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_2 = decrypt(key_str_2.as_bytes(), encrypted_2, nonce_2);

        let (encrypted_3, nonce_3) = encrypt(key_str_1.as_bytes(), plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_3 = decrypt(key_str_2.as_bytes(), encrypted_3, nonce_3);

        let (encrypted_4, nonce_4) = encrypt(key_str_1.as_bytes(), plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_4 = decrypt(key_str_2.as_bytes(), encrypted_4, nonce_4);

        let (encrypted_5, nonce_5) = encrypt(key_str_1.as_bytes(), plaintext_5.clone()).expect("encrypt 5 falhou");
        let decrypted_5 = decrypt(key_str_2.as_bytes(), encrypted_5, nonce_5);

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

        let (encrypted_1, nonce_1) = encrypt(key_str.as_bytes(), plaintext_1.clone()).expect("encrypt 1 falhou");
        let decrypted_1 = decrypt(key_str.as_bytes(), encrypted_1, nonce_1).expect("decrypt 1 falhou");

        let (encrypted_2, nonce_2) = encrypt(key_str.as_bytes(), plaintext_2.clone()).expect("encrypt 2 falhou");
        let decrypted_2 = decrypt(key_str.as_bytes(), encrypted_2, nonce_2).expect("decrypt 2 falhou");

        let (encrypted_3, nonce_3) = encrypt(key_str.as_bytes(), plaintext_3.clone()).expect("encrypt 3 falhou");
        let decrypted_3 = decrypt(key_str.as_bytes(), encrypted_3, nonce_3).expect("decrypt 3 falhou");

        let (encrypted_4, nonce_4) = encrypt(key_str.as_bytes(), plaintext_4.clone()).expect("encrypt 4 falhou");
        let decrypted_4 = decrypt(key_str.as_bytes(), encrypted_4, nonce_4).expect("decrypt 4 falhou");

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

        let (encrypted_1, nonce_1) = encrypt(key_str.as_bytes(), plaintext_1.clone()).expect("encrypt 1 falhou");
        let (encrypted_2, nonce_2) = encrypt(key_str.as_bytes(), plaintext_2.clone()).expect("encrypt 2 falhou");
        let (encrypted_3, nonce_3) = encrypt(key_str.as_bytes(), plaintext_3.clone()).expect("encrypt 3 falhou");
        let (encrypted_4, nonce_4) = encrypt(key_str.as_bytes(), plaintext_4.clone()).expect("encrypt 4 falhou");

        let decrypted_1 = decrypt(key_str.as_bytes(), encrypted_1, nonce_4);
        let decrypted_2 = decrypt(key_str.as_bytes(), encrypted_2, nonce_1);
        let decrypted_3 = decrypt(key_str.as_bytes(), encrypted_3, nonce_2);
        let decrypted_4 = decrypt(key_str.as_bytes(), encrypted_4, nonce_3);

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

    #[test]
    fn test_generate_salt_failure() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_simple_hash_success() {
        let key1 = "Teste".as_bytes();
        let key2 = "Testando".as_bytes();
        let key3 = "Coisa linda".as_bytes();
        let key4 = "Warhammer".as_bytes();

        let data1 = "numero 1".to_string();
        let data2 = "para ver se deu certo".to_string();
        let data3 = "da minha vida".to_string();
        let data4 = "Fantasy".to_string();

        let result1 = simple_hash(key1.clone(), data1.clone());
        let result2 = simple_hash(key2.clone(), data2.clone());
        let result3 = simple_hash(key3.clone(), data3.clone());
        let result4 = simple_hash(key4.clone(), data4.clone());
        
        let result11 = simple_hash(key1.clone(), data1.clone());
        let result22 = simple_hash(key2.clone(), data2.clone());
        let result33 = simple_hash(key3.clone(), data3.clone());
        let result44 = simple_hash(key4.clone(), data4.clone());

        assert_eq!(result1, result11);
        assert_eq!(result2, result22);
        assert_eq!(result3, result33);
        assert_eq!(result4, result44);
    }

    #[test]
    fn test_simple_hash_failure() {
        let key1 = "Teste".as_bytes();
        let key2 = "Testando".as_bytes();
        let key3 = "Coisa linda".as_bytes();
        let key4 = "Warhammer".as_bytes();

        let data1 = "numero 1".to_string();
        let data2 = "para ver se deu certo".to_string();
        let data3 = "da minha vida".to_string();
        let data4 = "Fantasy".to_string();

        let result11 = simple_hash(key1.clone(), data1.clone());
        let result22 = simple_hash(key2.clone(), data2.clone());
        let result33 = simple_hash(key3.clone(), data3.clone());
        let result44 = simple_hash(key4.clone(), data4.clone());
        
        let result21 = simple_hash(key2.clone(), data1.clone());
        let result32 = simple_hash(key3.clone(), data2.clone());
        let result43 = simple_hash(key4.clone(), data3.clone());
        let result14 = simple_hash(key1.clone(), data4.clone());

        assert_ne!(result11, result21);
        assert_ne!(result22, result32);
        assert_ne!(result33, result43);
        assert_ne!(result44, result14);
    }
}
