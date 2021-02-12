use super::digest::Digest;

use std::collections::HashMap;
use chrono::{DateTime, FixedOffset};

struct ImageConfig {
    // User defines the username or UID which the process in the container should run as.
	user: String,

	// ExposedPorts a set of ports to expose from a container running this image.
	exposed_ports: HashMap<String, String>, // TODO: change later map[string]struct{} `json:"ExposedPorts,omitempty"`

	// Env is a list of environment variables to be used in a container.
	env: Vec<String>,

	// Entrypoint defines a list of arguments to use as the command to execute when the container starts.
	entry_point: Vec<String>,

	// Cmd defines the default arguments to the entrypoint of the container.
	cmd: Vec<String>,

	// Volumes is a set of directories describing where the process is likely write data specific to a container instance.
	volumes: HashMap<String, String>,

	// WorkingDir sets the current working directory of the entrypoint process in the container.
	working_dir: String,

	// Labels contains arbitrary metadata for the container.
	labels: HashMap<String, String>,

	// StopSignal contains the system call signal that will be sent to the container to exit.
	stop_signal: String,
}

// RootFS describes a layer content addresses
struct RootFS {
	// Type is the type of the rootfs.
	typ: String,

	// DiffIDs is an array of layer content hashes (DiffIDs), in order from bottom-most to top-most.
	diff_ids: Vec<Digest>,
}

// History describes the history of a layer.
struct History {
	// Created is the combined date and time at which the layer was created, formatted as defined by RFC 3339, section 5.6.
	created: DateTime,

	// CreatedBy is the command which created the layer.
	created_by: String,

	// Author is the author of the build point.
	author: String,

	// Comment is a custom message set when creating the layer.
	comment: String,

	// EmptyLayer is used to mark if the history item created a filesystem diff.
	empty_layer: bool,
}

// Image is the JSON structure which describes some basic information about the image.
// This provides the `application/vnd.oci.image.config.v1+json` mediatype when marshalled to JSON.
struct Image {
	// Created is the combined date and time at which the image was created, formatted as defined by RFC 3339, section 5.6.
	created: DateTime,

	// Author defines the name and/or email address of the person or entity which created and is responsible for maintaining the image.
	author: String,

	// Architecture is the CPU architecture which the binaries in this image are built to run on.
	architecture: String,

	// OS is the name of the operating system which the image is built to run on.
	os: String,

	// Config defines the execution parameters which should be used as a base when running a container using the image.
	config: ImageConfig,

	// RootFS references the layer content addresses used by the image.
	root_fs: RootFS,

	// History describes the history of each layer.
	history: Vec<History>,
}
