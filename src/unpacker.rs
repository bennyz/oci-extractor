use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;
use tar::Entry;

use crate::spec::digest::Algorithm;
use crate::spec::layout::BLOBS;
use crate::spec::manifest::Manifest;

use super::spec::index::{Index, INDEX_FILE_NAME};

const WHITEOUT_PREFIX: &str = ".wh.";
const WHITEOUT_OPAQUE: &str = ".wh..wh..opq";

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
        // TODO: add validation for layout file
        let path = format!("{}/{}", self.image_path.as_str(), INDEX_FILE_NAME);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: Index = serde_json::from_reader(reader)?;

        // TODO: find a sane place for this
        fs::create_dir(&self.destination)?;

        for manifest in &index.manifests {
            self.parse_digest(&manifest.digest.algorithm, &manifest.digest.encoded)?;
        }

        Ok(index)
    }

    fn parse_digest(&self, algorithm: &Algorithm, encoded: &str) -> anyhow::Result<()> {
        let blob_path = format!("{}/{}/{}", self.image_path.as_str(), BLOBS, algorithm);
        let path = format!("{}/{}", &blob_path, &encoded);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let manifest: Manifest = serde_json::from_reader(reader)?;

        for layer in manifest.layers {
            println!("upacking layer: {:?}", &layer.digest.encoded);
            self.unpack_layer(&blob_path, &layer.digest.encoded)?;
        }

        Ok(())
    }

    fn unpack_layer(&self, layer_path: &str, layer: &str) -> anyhow::Result<()> {
        let path = format!("{}/{}", &layer_path, layer);
        let file = File::open(&path)?;

        // TODO: GzDecoder is not necessarily correct, a robust solution
        // would be to read the layer's media type
        let mut archive = Archive::new(GzDecoder::new(file));
        let destination = Path::new(&self.destination);

        archive.entries()?.filter_map(|e| e.ok()).for_each(|entry| {
            self.unpack_entry(destination, entry);
        });

        Ok(())
    }

    fn unpack_entry<T: std::io::Read>(&self, destination: &Path, mut entry: Entry<T>) {
        let path: PathBuf = entry.path().unwrap().to_path_buf();
        let last_component = path
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();
        if entry.header().entry_type().is_dir() {
            if !destination.join(&path).exists() {
                fs::create_dir(destination.join(&path)).unwrap();
            }
        } else if !last_component.starts_with(WHITEOUT_PREFIX) {
            entry.unpack_in(destination).unwrap();
        } else if last_component.starts_with(WHITEOUT_OPAQUE) {
            fs::remove_dir_all(path.parent().unwrap()).unwrap();
        }
    }
}
