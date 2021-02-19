use std::{
    fs::File,
    io::{BufReader, Read},
};

use super::spec::index::{Index, INDEX_FILE_NAME};

#[derive(Debug)]
pub struct Unpacker {
    image_name: String,
    bundle_path: String,
}

impl Unpacker {
    pub fn new(image_name: String, bundle_path: String) -> Self {
        Unpacker {
            image_name,
            bundle_path,
        }
    }

    pub fn unpack(&self) {
        let engine = Engine::new(self.image_name.to_owned());
        print!("{:?}", engine.parse().unwrap());
    }
}

struct Engine {
    image_path: String,
}

impl Engine {
    pub fn new(image_path: String) -> Self {
        Engine { image_path }
    }

    pub fn parse(&self) -> anyhow::Result<Index> {
        // TODO: add validation for layout file
        let path = format!("{}/{}", self.image_path.as_str(), INDEX_FILE_NAME);
        println!("parsing image: {}", path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: Index = serde_json::from_reader(reader)?;
        Ok(index)
    }
}
