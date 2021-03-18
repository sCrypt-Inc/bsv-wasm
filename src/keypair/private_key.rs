use std::borrow::Borrow;

use bitcoin_hashes::{Hash, hex::ToHex};
use wasm_bindgen::throw_str;
use wasm_bindgen::prelude::*;
use k256::{EncodedPoint, SecretKey};
use rand_core::OsRng;


#[wasm_bindgen]
#[derive(Debug)]
pub struct PrivateKey {
  secret_key: SecretKey
}
 
/**
 * Serialisation methods
 */
#[wasm_bindgen]
impl PrivateKey {
  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> String {
      let secret_key_bytes = self.secret_key.to_bytes().to_vec();
      hex::encode(secret_key_bytes)
  }

  pub fn get_point(&self, compressed: bool) -> Vec<u8> {
    EncodedPoint::from_secret_key(&self.secret_key, compressed).as_bytes().into()
  }

  pub fn to_wif(&self, compressed: bool) -> String {
    // 1. Get Private Key hex
    let priv_key_hex = self.to_hex();

    // 2. Add 0x80 in front + 0x01 to end if compressed pub key
    let padded_hex = match compressed {
      true => format!("80{}01", priv_key_hex),
      false => format!("80{}", priv_key_hex)
    };

    // 3. SHA256d
    let bytes =  match hex::decode(padded_hex.clone()) {
      Ok(v) => v,
      Err(e) => wasm_bindgen::throw_str(&e.to_string())
    };

    let shad_hex = bitcoin_hashes::sha256d::Hash::hash(&bytes);

    // 4. Take first 4 bytes as checksum
    let checksum = shad_hex.to_vec()[0..4].to_hex();

    // 5. Add checksum to end of padded private key
    let extended_key = format!("{}{}", padded_hex, checksum);

    // 6 Base58 Result
    let extended_key_bytes = match hex::decode(extended_key) {
      Ok(v) => v,
      Err(e) => wasm_bindgen::throw_str(&e.to_string())
    };

    bs58::encode(extended_key_bytes).into_string()
  }
}

/**
 * Deserialisation Methods
 */
#[wasm_bindgen]
impl PrivateKey {
  #[wasm_bindgen(js_name = fromRandom)]
  pub fn from_random() -> PrivateKey {
    let secret_key = k256::SecretKey::random(&mut OsRng);

    PrivateKey{
      secret_key
    }
  }

  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: String) -> Result<PrivateKey, JsValue> {
    let bytes = match hex::decode(hex_str) {
      Ok(bytes) => bytes,
      Err(e) => throw_str(&e.to_string())
    };
 
    let secret_key = match SecretKey::from_bytes(bytes) {
      Ok(key) => key,
      Err(e) => throw_str(&e.to_string())
    };

    Ok(PrivateKey{
      secret_key
    })
  }

  #[wasm_bindgen(js_name = fromWIF)]
  pub fn from_wif(wif_string: String) -> Result<PrivateKey, JsValue> {
    // 0. Check if compressed
    let is_compressed = wif_string.starts_with("5");

    // 1. Decode from Base58
    let wif_bytes = match bs58::decode(wif_string).into_vec() {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    let wif_without_checksum = wif_bytes[0..wif_bytes.len()-4].to_vec();

    // 2. Check the Checksum
    let checksum = wif_bytes[wif_bytes.len()-4..].to_hex();
    let check_hash = bitcoin_hashes::sha256d::Hash::hash(&wif_without_checksum);
    let check_string = check_hash.to_vec()[0..4].to_hex();
    
    if check_string.ne(&checksum) {
      throw_str("Checksum does not match! Invalid WIF");
    }

    // 3. Check if compressed public key, return private key string
    let private_key_hex = match is_compressed {
      true => wif_without_checksum[1..].to_hex(),
      false => wif_without_checksum[1..].to_hex()
    };

    PrivateKey::from_hex(private_key_hex.into())
  }
}