extern crate openssl;
use openssl::{
    encrypt::{Decrypter, Encrypter},
    error::ErrorStack,
    pkey::PKey,
    rsa::{Padding, Rsa},
};

pub type EncryptionResult<T> = Result<T, ErrorStack>;

pub fn generate_private_public_key_pair() -> EncryptionResult<(Vec<u8>, Vec<u8>)> {
    let rsa = Rsa::generate(2048)?;
    let private_key = PKey::from_rsa(rsa)?;
    let public_key_pem = private_key.public_key_to_pem()?;
    let private_key_pem = private_key.private_key_to_pem_pkcs8()?;
    Ok((private_key_pem, public_key_pem))
}

pub fn encrypt(public_key_bytes: &[u8], pass: &[u8]) -> EncryptionResult<Vec<u8>> {
    let public_key = PKey::public_key_from_pem(public_key_bytes)?;
    let mut rsa_encryptor = Encrypter::new(&public_key)?;
    rsa_encryptor.set_rsa_padding(Padding::PKCS1_OAEP)?;

    let buffer_len = rsa_encryptor.encrypt_len(pass)?;
    let mut encrypted_buffer = vec![0; buffer_len];

    let encrypted_length = rsa_encryptor.encrypt(pass, &mut encrypted_buffer)?;
    encrypted_buffer.truncate(encrypted_length);
    Ok(encrypted_buffer)
}

pub fn decrypt(private_key_bytes: &[u8], encrypted_pass: &[u8]) -> EncryptionResult<Vec<u8>> {
    let private_key = PKey::private_key_from_pem(private_key_bytes)?;
    let mut decryptor = Decrypter::new(&private_key)?;
    decryptor.set_rsa_padding(Padding::PKCS1_OAEP)?;

    let buffer_len = decryptor.decrypt_len(encrypted_pass)?;
    let mut decrypted_buffer = vec![0; buffer_len];

    let decrypted_length = decryptor.decrypt(encrypted_pass, &mut decrypted_buffer)?;
    decrypted_buffer.truncate(decrypted_length);

    Ok(decrypted_buffer)
}
