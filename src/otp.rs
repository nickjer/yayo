use anyhow::{anyhow, Context, Result};
use data_encoding::BASE32_NOPAD;
use ring::hmac;

pub struct Otp<'a> {
  secret: &'a str,
  algorithm: &'a str,
  digits: usize,
  step: u64,
}

impl<'a> Otp<'a> {
  pub fn new(
    secret: &'a str,
    algorithm: &'a str,
    digits: usize,
    step: u64,
  ) -> Self {
    Self {
      secret,
      algorithm,
      digits,
      step,
    }
  }

  pub fn get_code(&self, time: u64) -> Result<String> {
    let counter = time / self.step;
    let message = counter.to_be_bytes();
    let signing_key = hmac::Key::new(self.algorithm(), &self.decode_secret()?);
    let digest = hmac::sign(&signing_key, &message);
    self.encode_digest(digest.as_ref())
  }

  fn algorithm(&self) -> hmac::Algorithm {
    match self.algorithm {
      "SHA256" => hmac::HMAC_SHA256,
      "SHA512" => hmac::HMAC_SHA512,
      _ => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
    }
  }

  fn encode_digest(&self, digest: &[u8]) -> Result<String> {
    let offset =
      (digest.last().ok_or_else(|| anyhow!("Digest is empty"))? & 0xf) as usize;
    let snum = u32::from_be_bytes([
      digest[offset],
      digest[offset + 1],
      digest[offset + 2],
      digest[offset + 3],
    ]) & 0x7fff_ffff;
    let hotp_code = snum % (10_u32).pow(self.digits as u32);
    Ok(format!("{:0length$}", hotp_code, length = self.digits))
  }

  fn decode_secret(&self) -> Result<Vec<u8>> {
    BASE32_NOPAD
      .decode(self.secret.as_bytes())
      .context("Failed to decode secret")
  }
}
