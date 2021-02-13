use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::descriptor::Descriptor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Index {
    pub schema_version: u32,

    // Manifests references platform specific manifests.
    pub manifests: Vec<Descriptor>,

    // Annotations contains arbitrary metadata for the image index.
    pub annotations: Option<HashMap<String, String>>,
}

mod tests {
    use crate::spec::{
        descriptor::Platform,
        digest::{Algorithm, Digest},
        media_types::MediaType,
    };

    use super::*;

    #[test]
    fn test_deserialize_index() {
        const INDEX_JSON: &str = r#"
        {
            "schemaVersion": 2,
            "manifests": [
              {
                "mediaType": "application/vnd.oci.image.manifest.v1+json",
                "size": 7143,
                "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
                "platform": {
                  "architecture": "ppc64le",
                  "os": "linux"
                }
              },
              {
                "mediaType": "application/vnd.oci.image.manifest.v1+json",
                "size": 7682,
                "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
                "platform": {
                  "architecture": "amd64",
                  "os": "linux"
                }
              }
            ],
            "annotations": {
              "com.example.key1": "value1",
              "com.example.key2": "value2"
            }
          }"#;

        let mut annotations: HashMap<String, String> = HashMap::new();
        annotations.insert(String::from("com.example.key1"), String::from("value1"));
        annotations.insert(String::from("com.example.key2"), String::from("value2"));

        let descriptor1 = Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::new(
                Algorithm::Sha256,
                String::from("e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f"),
            ),
            annotations: None,
            size: 7143,
            urls: None,
            platform: Some(Platform {
                architecture: String::from("ppc64le"),
                os: String::from("linux"),
                os_features: None,
                os_version: None,
                variant: None,
            }),
            data: None,
        };

        let descriptor2 = Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::new(
                Algorithm::Sha256,
                String::from("5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"),
            ),
            annotations: None,
            size: 7682,
            urls: None,
            platform: Some(Platform {
                architecture: String::from("amd64"),
                os: String::from("linux"),
                os_features: None,
                os_version: None,
                variant: None,
            }),
            data: None,
        };

        let index: Index = serde_json::from_str(INDEX_JSON).unwrap();
        let expected = Index {
            schema_version: 2,
            manifests: vec![descriptor1, descriptor2],
            annotations: Some(annotations),
        };

        assert_eq!(index, expected)
    }
}
