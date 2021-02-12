use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Eq)]
struct Index {
    pub version: u32, 
    	// Manifests references platform specific manifests.
	pub manifests: Vec<Descriptor>,

	// Annotations contains arbitrary metadata for the image index.
	pub annotations: HashMap<String, String>

}