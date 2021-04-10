use std::collections::HashMap;

use super::descriptor::Descriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub schema_version: u32,

    // Config references a configuration object for a container, by digest.
    // The referenced configuration object is a JSON blob that the runtime uses to set up the container.
    pub config: Descriptor,

    // Layers is an indexed list of layers referenced by the manifest.
    pub layers: Vec<Descriptor>,

    // Annotations contains arbitrary metadata for the image manifest.
    pub annotations: Option<HashMap<String, String>>,
}

mod tests {
    use crate::spec::{
        descriptor::Descriptor,
        digest::{Algorithm, Digest},
        media_types::MediaType,
    };

    use super::*;

    #[test]
    fn test_deseralize_manifest() {
        const MANIFEST_JSON: &str = r#"
		{
			"schemaVersion": 2,
			"config": {
			  "mediaType": "application/vnd.oci.image.config.v1+json",
			  "size": 7023,
			  "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"
			},
			"layers": [
			  {
				"mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
				"size": 32654,
				"digest": "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0"
			  },
			  {
				"mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
				"size": 16724,
				"digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b"
			  },
			  {
				"mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
				"size": 73109,
				"digest": "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736"
			  }
			],
			"annotations": {
			  "com.example.key1": "value1",
			  "com.example.key2": "value2"
			}
		  }"#;
        let config = Descriptor {
            media_type: MediaType::ImageConfig,
            digest: Digest::new(
                Algorithm::Sha256,
                String::from("b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"),
            ),
            annotations: None,
            size: 7023,
            urls: None,
            platform: None,
            data: None,
        };
        let layers = vec![
            Descriptor {
                media_type: MediaType::ImageLayerTarGzip,
                digest: Digest::new(
                    Algorithm::Sha256,
                    String::from(
                        "9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0",
                    ),
                ),
                annotations: None,
                size: 32654,
                urls: None,
                platform: None,
                data: None,
            },
            Descriptor {
                media_type: MediaType::ImageLayerTarGzip,
                digest: Digest::new(
                    Algorithm::Sha256,
                    String::from(
                        "3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b",
                    ),
                ),
                annotations: None,
                size: 16724,
                urls: None,
                platform: None,
                data: None,
            },
            Descriptor {
                media_type: MediaType::ImageLayerTarGzip,
                digest: Digest::new(
                    Algorithm::Sha256,
                    String::from(
                        "ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736",
                    ),
                ),
                annotations: None,
                size: 73109,
                urls: None,
                platform: None,
                data: None,
            },
        ];
        let mut annotations: HashMap<String, String> = HashMap::new();
        annotations.insert(String::from("com.example.key1"), String::from("value1"));
        annotations.insert(String::from("com.example.key2"), String::from("value2"));

        let manifest: Manifest = serde_json::from_str(MANIFEST_JSON).unwrap();
        let expected = Manifest {
            schema_version: 2,
            config,
            layers,
            annotations: Some(annotations),
        };

        assert_eq!(manifest, expected);
    }
}
