use std::{fmt, str::FromStr};

use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Algorithm {
    Sha256,
    Sha512,
    Unregistered(String),
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algorithm::Sha256 => write!(f, "sha256"),
            Algorithm::Sha512 => write!(f, "sha512"),
            Algorithm::Unregistered(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha256" => Ok(Algorithm::Sha256),
            "sha512" => Ok(Algorithm::Sha512),
            _ => Ok(Algorithm::Unregistered(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Digest {
    pub algorithm: Algorithm,
    pub encoded: String,
}

impl Digest {
    pub fn new(algorithm: Algorithm, encoded: String) -> Self {
        Self { algorithm, encoded }
    }

    pub fn validate(&self) -> Result<()> {
        todo!()
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.encoded)
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let split: Vec<&str> = s.split(':').collect();

        return Ok(Digest::new(
            Algorithm::from_str(split[0]).unwrap(),
            split[1].to_owned(),
        ));
    }
}
