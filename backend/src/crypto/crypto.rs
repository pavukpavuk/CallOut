use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng}, // Or `Aes128Gcm`
    Aes256Gcm,
    Key,
    KeySizeUser,
    Nonce,
};
use hex;

pub struct EncryptedData {
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

#[allow(dead_code)]
impl EncryptedData {
    pub fn nonce_hex(&self) -> String {
        hex::encode(self.nonce.clone())
    }
    pub fn ciphertext_hex(&self) -> String {
        hex::encode(self.ciphertext.clone())
    }
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    pub fn nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}
#[allow(dead_code, unused_imports)]
#[derive(Debug)]
pub struct DecryptedData {
    decrypted_ciphertext: Vec<u8>,
}

#[allow(dead_code, unused_imports)]
impl DecryptedData {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.decrypted_ciphertext.clone()
    }

    pub fn as_utf8(&self) -> Result<String, String> {
        String::from_utf8(self.decrypted_ciphertext.clone()).map_err(|e| e.to_string())
    }

    pub fn as_hex(&self) -> String {
        hex::encode(self.decrypted_ciphertext.clone())
    }
}

pub struct Crypto {
    key: Key<Aes256Gcm>,
    cipher: Aes256Gcm,
}

#[allow(dead_code)]
impl Crypto {
    pub fn key_hex(&self) -> String {
        hex::encode(self.key.to_vec())
    }
    pub fn key(&self) -> Vec<u8> {
        self.key.clone().to_vec()
    }

    fn generate_key_to_vec() -> Vec<u8> {
        let key = Aes256Gcm::generate_key(OsRng);
        key.to_vec()
    }

    fn generate_key_to_key() -> Key<Aes256Gcm> {
        Aes256Gcm::generate_key(OsRng)
    }

    pub fn new_with_provided_key_hex(key: &[u8]) -> Result<Self, String> {
        let key = hex::decode(key).map_err(|e| format!("Decoding error (key): {}", e))?;

        if key.len() != Aes256Gcm::key_size() {
            return Err(format!(
                "Key is wrong size: expected {} bytes, received {} bytes",
                Aes256Gcm::key_size(),
                key.len()
            ));
        }

        let key = Key::<Aes256Gcm>::clone_from_slice(&key);
        let cipher = Aes256Gcm::new(&key);

        Ok(Self { key, cipher })
    }

    pub fn new_with_random_key() -> Self {
        let key = Self::generate_key_to_key();
        let cipher = Aes256Gcm::new(&key);

        Self { key, cipher }
    }

    pub fn new_with_provided_key(key: &[u8]) -> Result<Self, String> {
        if key.len() != Aes256Gcm::key_size() {
            return Err(format!(
                "Key is wrong size: expected {} bytes, received {} bytes",
                Aes256Gcm::key_size(),
                key.len()
            ));
        }

        let key = Key::<Aes256Gcm>::clone_from_slice(&key);
        let cipher = Aes256Gcm::new(&key);

        Ok(Self { key, cipher })
    }

    pub fn encrypt(&self, input: &[u8]) -> Result<EncryptedData, String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        match self.cipher.encrypt(&nonce, input) {
            Ok(ciphertext) => Ok(EncryptedData {
                nonce: nonce.to_vec(),
                ciphertext: ciphertext.to_vec(),
            }),
            Err(_) => {
                // purposely obtuse errors so no need to propogate
                Err("Error encrypting".to_string())
            }
        }
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<DecryptedData, String> {
        let nonce = Nonce::from_slice(&nonce);

        match self.cipher.decrypt(&nonce, ciphertext) {
            Ok(text) => Ok(DecryptedData {
                decrypted_ciphertext: text,
            }),
            Err(_) => Err("Error decrypting".to_string()),
        }
    }
}
