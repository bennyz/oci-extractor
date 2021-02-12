use std::collections::HashMap;

use super::descriptor::Descriptor;

struct Manifest {

    // Config references a configuration object for a container, by digest.
	// The referenced configuration object is a JSON blob that the runtime uses to set up the container.
	config: Descriptor,

	// Layers is an indexed list of layers referenced by the manifest.
	layers: Vec<Descriptor>,

	// Annotations contains arbitrary metadata for the image manifest.
	annotations: HashMap<String, String>,
}