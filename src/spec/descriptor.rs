use std::collections::HashMap;
use super::media_types::MediaType;
use super::digest::Digest;

pub struct Descriptor {
    pub media_type: MediaType,
    pub digest: Digest,
    pub size: u64,
    pub urls: Vec<String>,
    pub annotations: HashMap<String, String>,
    pub platform: Option<Platform>,
    pub data: Option<String>,
}

pub struct Platform {
	// Architecture field specifies the CPU architecture, for example
	// `amd64` or `ppc64`.
	architecture: String,

	// OS specifies the operating system, for example `linux` or `windows`.
	os: String,

	// OSVersion is an optional field specifying the operating system
	// version, for example on Windows `10.0.14393.1066`.
	os_version: String,

	// OSFeatures is an optional field specifying an array of strings,
	// each listing a required OS feature (for example on Windows `win32k`).
	os_features: Vec<String>,

	// Variant is an optional field specifying a variant of the CPU, for
	// example `v7` to specify ARMv7 when architecture is `arm`.
	variant: String
}
