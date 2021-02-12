use anyhow::{anyhow, Error, Result};
use sha2::{Sha256, Sha512};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Algorithm {
    Sha256,
    Sha512,
    Unregistered(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest {
    algorithm: Algorithm,
    encoded: String,
}

impl Digest {

    pub fn new(algorithm: Algorithm, encoded: String) -> Self {
        Self {
            algorithm,
            encoded,
        }
    }

    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}