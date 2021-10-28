use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::digest::Digest;
use super::media_types::MediaType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    pub media_type: MediaType,
    pub digest: Digest,
    pub size: u64,
    pub urls: Option<Vec<String>>,
    pub annotations: Option<HashMap<String, String>>,
    pub platform: Option<Platform>,
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Platform {
    // Architecture field specifies the CPU architecture, for example
    // `amd64` or `ppc64`.
    // TODO: use enum
    pub architecture: String,

    // OS specifies the operating system, for example `linux` or `windows`.
    // TODO: use enum
    pub os: String,

    // OSVersion is an optional field specifying the operating system
    // version, for example on Windows `10.0.14393.1066`.
    pub os_version: Option<String>,

    // OSFeatures is an optional field specifying an array of strings,
    // each listing a required OS feature (for example on Windows `win32k`).
    pub os_features: Option<Vec<String>>,

    // Variant is an optional field specifying a variant of the CPU, for
    // example `v7` to specify ARMv7 when architecture is `arm`.
    // TODO: use enum
    pub variant: Option<String>,
}

mod tests {
    use crate::spec::{
        descriptor::Descriptor,
        digest::{Algorithm, Digest},
        media_types::MediaType,
    };

    #[test]
    fn test_deseriaize_descriptor() {
        const DESCRIPTOR_JSON: &str = r#"
        {
            "mediaType": "application/vnd.oci.image.manifest.v1+json",
            "size": 7682,
            "digest": "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270",
            "urls": [
              "https://example.com/example-manifest"
            ]
        }"#;

        let descriptor: Descriptor = serde_json::from_str(DESCRIPTOR_JSON).unwrap();
        let expected = Descriptor {
            media_type: MediaType::ImageManifest,
            digest: Digest::new(
                Algorithm::Sha256,
                String::from("5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"),
            ),
            annotations: None,
            size: 7682,
            urls: Some(vec![String::from("https://example.com/example-manifest")]),
            platform: None,
            data: None,
        };

        assert_eq!(descriptor, expected)
    }
}
