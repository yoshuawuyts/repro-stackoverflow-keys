extern crate base64;
extern crate openssl;

use openssl::rsa::{Padding, Rsa};
use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
  let data = b"hello world";
  let pub_key = fs::read_to_string("./key.pem")?; // Read a public key.
  let rsa = Rsa::public_key_from_pem(pub_key.as_bytes())?;

  let mut res = vec![0; rsa.size() as usize];
  rsa.public_encrypt(data, &mut res, Padding::PKCS1)?;

  let encrypted = base64::encode(&res);
  println!("Encrypted Data: \n{}", encrypted);
  Ok(())
}
