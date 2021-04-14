use std::{fs::{self, File}, io::BufReader, path::PathBuf};

use crate::spec::digest::Algorithm;
use crate::spec::layout::BLOBS;
use crate::spec::manifest::Manifest;

use super::spec::index::{Index, INDEX_FILE_NAME};

#[derive(Debug)]
pub struct Unpacker {
    image_name: String,
    destination: String,
}

impl Unpacker {
    pub fn new(image_name: String, destination: String) -> Self {
        Unpacker {
            image_name,
            destination,
        }
    }

    pub fn unpack(&self) {
        let engine = Engine::new(self.image_name.to_owned(), self.destination.to_owned());
        engine.parse().unwrap();
    }
}

struct Engine {
    image_path: String,
    destination: String,
}

impl Engine {
    pub fn new(image_path: String, destination: String) -> Self {
        Engine {
            image_path,
            destination,
        }
    }

    pub fn parse(&self) -> anyhow::Result<Index> {
        // TODO: find a sane place for this
        fs::create_dir(&self.destination)?;

        // TODO: add validation for layout file
        let path = format!("{}/{}", self.image_path.as_str(), INDEX_FILE_NAME);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: Index = serde_json::from_reader(reader)?;

        for manifest in &index.manifests {
            println!("{:?}", manifest.annotations);
            println!("digest: {:?}", manifest.digest);
            println!(
                "read config for digest {}:{}",
                manifest.digest.algorithm, manifest.digest.encoded
            );
            self.parse_digest(&manifest.digest.algorithm, &manifest.digest.encoded)?;
        }

        Ok(index)
    }

    fn parse_digest(&self, algorithm: &Algorithm, encoded: &str) -> anyhow::Result<()> {
        let blob_path = format!("{}/{}/{}", self.image_path.as_str(), BLOBS, algorithm);
        let path = format!("{}/{}", &blob_path, &encoded);
        println!("path {:?}", path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let manifest: Manifest = serde_json::from_reader(reader)?;
        for layer in manifest.layers {
            println!("layer {:?}", layer);
            println!("layer digest {:?}", layer.digest.encoded);
            self.parse_tar(&blob_path, &layer.digest.encoded)?;
        }
        Ok(())
    }

    fn parse_tar(&self, layer_path: &str, layer: &str) -> anyhow::Result<()> {
        use flate2::read::GzDecoder;
        use tar::Archive;

        let path = format!("{}/{}", &layer_path, layer);
        println!("opening file: {:?}", &path);
        let file = File::open(&path)?;

        // TODO: GzDecoder is not necessarily correct, a robust solution
        // would be to read the layer's media type
        let mut archive = Archive::new(GzDecoder::new(file));

        println!("Extracting the following files:");
        archive
            .entries()?
            .filter_map(|e| e.ok())
            .map(|mut entry| -> anyhow::Result<PathBuf> {
                let path = entry.path()?.into_owned();
                entry.unpack_in(&self.destination)?;
                Ok(path)
            })
            .filter_map(|e| e.ok())
            .for_each(|x| println!("> {}", x.display()));

        Ok(())
    }
}
