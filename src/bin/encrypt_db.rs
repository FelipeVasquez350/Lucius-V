use std::fs::{self};
use chacha20poly1305::{XChaCha20Poly1305, aead::{NewAead, Aead}};
use dotenv::dotenv;

pub fn encrypt_file(input_path: &str, output_path: &str, key: &[u8; 32], nonce: &[u8; 24]) -> std::io::Result<()> {
  let cipher = XChaCha20Poly1305::new(key.into());
  let file_data = fs::read(input_path)?;
  let encrypted_data = cipher
    .encrypt(nonce.into(), file_data.as_ref())
    .map_err(|err| println!("Encrypting file: {}", err));
  
  fs::write(output_path, encrypted_data.unwrap())?;
  Ok(())
}
  
    

fn main() {
  dotenv().ok();

  let str_key = std::env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
  let mut key = [0; 32];
  for (i, c) in str_key.chars().enumerate() {
      key[i] = c as u8;
  }
  
  let str_nonce = std::env::var("ENCRYPTION_NONCE").expect("ENCRYPTION_NONCE must be set");
  let mut nonce = [0; 24];
  for (i, c) in str_nonce.chars().enumerate() {
    nonce[i] = c as u8;
  }
  
  encrypt_file("src/db/db.sqlite3", "src/db/db.enc", &key, &nonce).unwrap();
}