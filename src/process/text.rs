use std::{collections::HashMap, io::Read};

use super::generate_password;
use crate::TextSignFormat;
use anyhow::Result;
use ed25519_dalek::{ed25519::signature::Signer, Signature, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
    pub fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        // generate a random 32-byte key
        let key = generate_password(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = blake3::keyed_hash(&self.key, &buf);
        println!("sign trait signature: {:?}", signature);
        Ok(signature.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        let cnt = reader.read_to_end(&mut buf)?;
        println!("cnt: {}", cnt);
        let ret = blake3::keyed_hash(&self.key, &buf);
        for (i, (sig_byte, ret_byte)) in signature.iter().zip(ret.as_bytes()).enumerate() {
            if sig_byte != ret_byte {
                println!(
                    "Byte {} differs: signature byte = {}, ret byte = {}",
                    i, sig_byte, ret_byte
                );
            }
        }
        Ok(signature == ret.as_bytes())
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Ed25519Signer::new(key))
    }

    pub fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        // generate a signing key
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        // derive a public key from the signing key
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());
        Ok(map)
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&signature[..64]).try_into()?;
        let sig = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

// use private key to sign text
pub fn process_text_sign(
    reader: &mut dyn Read,
    format: TextSignFormat,
    key: &[u8],
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };
    signer.sign(reader)
}

// use public key to verify text
pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    signature: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, signature)
}

// generate a private key, default cryptography algorithm is BLAKE3
pub fn process_text_generate_key(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");

    #[test]
    fn test_process_text_generate_key() -> Result<()> {
        // generate Blake3 key
        let b3_fmt = TextSignFormat::Blake3;
        let map = process_text_generate_key(b3_fmt)?;
        assert_eq!(map["blake3.txt"].len(), 32);

        // generate Ed25519 key
        let e25519_fmt = TextSignFormat::Ed25519;
        let map = process_text_generate_key(e25519_fmt)?;
        println!("ed25519 sk: {:?}", map["ed25519.sk"]);
        println!("ed25519 pk: {:?}", map["ed25519.pk"]);
        assert_eq!(map["ed25519.sk"].len(), 32);
        assert_eq!(map["ed25519.pk"].len(), 32);
        Ok(())
    }

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        // test sign and verify with Blake3
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let signature = process_text_sign(&mut reader, format, KEY)?;
        let verified = process_text_verify(&mut reader1, KEY, &signature, format)?;
        assert!(verified);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        // test sign and verify with Ed25519 pk and sk
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Ed25519;
        let pk = include_bytes!("../../fixtures/ed25519.pk");
        let sk = include_bytes!("../../fixtures/ed25519.sk");
        let signature = process_text_sign(&mut reader, format, sk)?;
        let verified = process_text_verify(&mut reader1, pk, &signature, format)?;
        assert!(verified);
        Ok(())
    }
}
