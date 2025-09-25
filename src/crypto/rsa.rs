use openssl::encrypt::{Decrypter, Encrypter};
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::{Padding, Rsa};

use crate::errors::CryptoError;
use crate::storage::fsio::write_all;
use std::path::Path;

fn map_err(ctx: &str, e: ErrorStack) -> CryptoError {
    CryptoError::OpenSsl(format!("{ctx}: {e}"))
}

pub fn load_public_key(pem: &str) -> Result<PKey<openssl::pkey::Public>, CryptoError> {
    if pem.contains("BEGIN RSA PUBLIC KEY") {
        let rsa =
            Rsa::public_key_from_pem_pkcs1(pem.as_bytes()).map_err(|e| map_err("pkcs1", e))?;
        PKey::from_rsa(rsa).map_err(|e| map_err("pkey from rsa", e))
    } else {
        PKey::public_key_from_pem(pem.as_bytes()).map_err(|e| map_err("public_key_from_pem", e))
    }
}

pub fn load_private_key(pem: &str, pass: Option<&[u8]>) -> Result<PKey<Private>, CryptoError> {
    if let Some(p) = pass {
        PKey::private_key_from_pem_passphrase(pem.as_bytes(), p)
            .map_err(|e| map_err("private+pass", e))
    } else {
        PKey::private_key_from_pem(pem.as_bytes()).map_err(|e| map_err("private", e))
    }
}

pub fn encrypt_with_pub(plain: &[u8], pub_pem: &str) -> Result<Vec<u8>, CryptoError> {
    let pkey = load_public_key(pub_pem)?;
    let mut enc = Encrypter::new(&pkey).map_err(|e| map_err("enc new", e))?;
    enc.set_rsa_padding(Padding::PKCS1_OAEP)
        .map_err(|e| map_err("set padding", e))?;
    enc.set_rsa_oaep_md(MessageDigest::sha256())
        .map_err(|e| map_err("oaep md", e))?;
    enc.set_rsa_mgf1_md(MessageDigest::sha256())
        .map_err(|e| map_err("mgf1 md", e))?;
    let mut out = vec![
        0;
        enc.encrypt_len(plain)
            .map_err(|e| map_err("encrypt_len", e))?
    ];
    let n = enc
        .encrypt(plain, &mut out)
        .map_err(|e| map_err("encrypt", e))?;
    out.truncate(n);
    Ok(out)
}

pub fn decrypt_with_priv(
    cipher: &[u8],
    priv_pem: &str,
    pass: Option<&[u8]>,
) -> Result<Vec<u8>, CryptoError> {
    let pkey = load_private_key(priv_pem, pass)?;
    let mut dec = Decrypter::new(&pkey).map_err(|e| map_err("dec new", e))?;
    dec.set_rsa_padding(Padding::PKCS1_OAEP)
        .map_err(|e| map_err("set padding", e))?;
    dec.set_rsa_oaep_md(MessageDigest::sha256())
        .map_err(|e| map_err("oaep md", e))?;
    dec.set_rsa_mgf1_md(MessageDigest::sha256())
        .map_err(|e| map_err("mgf1 md", e))?;
    let mut out = vec![
        0;
        dec.decrypt_len(cipher)
            .map_err(|e| map_err("decrypt_len", e))?
    ];
    let n = dec
        .decrypt(cipher, &mut out)
        .map_err(|e| map_err("decrypt", e))?;
    out.truncate(n);
    Ok(out)
}

pub fn generate_keys(priv_out: &str, pub_out: &str) -> Result<(), CryptoError> {
    let rsa = Rsa::generate(3072).map_err(|e| map_err("rsa gen", e))?;
    let pkey = PKey::from_rsa(rsa).map_err(|e| map_err("to pkey", e))?;
    let sk_pem = pkey
        .private_key_to_pem_pkcs8()
        .map_err(|e| map_err("pem pkcs8", e))?;
    let pk_pem = pkey
        .public_key_to_pem()
        .map_err(|e| map_err("pub pem", e))?;
    write_all(Path::new(priv_out), &sk_pem)
        .map_err(|e| CryptoError::OpenSsl(format!("write priv: {e}")))?;
    write_all(Path::new(pub_out), &pk_pem)
        .map_err(|e| CryptoError::OpenSsl(format!("write pub: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_encrypt_decrypt_pkcs8() {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let pk_pem = String::from_utf8(pkey.public_key_to_pem().unwrap()).unwrap();
        let sk_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let msg = b"hello, cryptoworld";
        let cipher = encrypt_with_pub(msg, &pk_pem).unwrap();
        let plain = decrypt_with_priv(&cipher, &sk_pem, None).unwrap();
        assert_eq!(msg, &plain[..]);
    }
}
