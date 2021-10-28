use super::digest::Digest;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageConfig {
    // User defines the username or UID which the process in the container should run as.
    pub user: Option<String>,

    // ExposedPorts a set of ports to expose from a container running this image.
    // TODO: change later to normal port key
    pub exposed_ports: Option<HashMap<String, Value>>,

    // Env is a list of environment variables to be used in a container.
    pub env: Option<Vec<String>>,

    // Entrypoint defines a list of arguments to use as the command to execute when the container starts.
    pub entrypoint: Option<Vec<String>>,

    // Cmd defines the default arguments to the entrypoint of the container.
    pub cmd: Option<Vec<String>>,

    // Volumes is a set of directories describing where the process is likely write data specific to a container instance.
    pub volumes: Option<HashMap<String, Value>>,

    // WorkingDir sets the current working directory of the entrypoint process in the container.
    pub working_dir: Option<String>,

    // Labels contains arbitrary metadata for the container.
    pub labels: Option<HashMap<String, String>>,

    // StopSignal contains the system call signal that will be sent to the container to exit.
    pub stop_signal: Option<String>,
}

// RootFS describes a layer content addresses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootFs {
    // Type is the type of the rootfs.
    // TODO: This has to be "layers", treat and validate accordingly
    #[serde(rename = "type")]
    pub typ: String,

    // DiffIDs is an array of layer content hashes (DiffIDs), in order from bottom-most to top-most.
    pub diff_ids: Vec<Digest>,
}

// History describes the history of a layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    // Created is the combined date and time at which the layer was created, formatted as defined by RFC 3339, section 5.6.
    pub created: Option<DateTime<FixedOffset>>,

    // CreatedBy is the command which created the layer.
    pub created_by: Option<String>,

    // Author is the author of the build point.
    pub author: Option<String>,

    // Comment is a custom message set when creating the layer.
    pub comment: Option<String>,

    // EmptyLayer is used to mark if the history item created a filesystem diff.
    pub empty_layer: Option<bool>,
}

// Image is the JSON structure which describes some basic information about the image.
// This provides the `application/vnd.oci.image.config.v1+json` mediatype when marshalled to JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    // Created is the combined date and time at which the image was created, formatted as defined by RFC 3339, section 5.6.
    pub created: Option<DateTime<FixedOffset>>,

    // Author defines the name and/or email address of the person or entity which created and is responsible for maintaining the image.
    pub author: Option<String>,

    // Architecture is the CPU architecture which the binaries in this image are built to run on.
    pub architecture: String,

    // OS is the name of the operating system which the image is built to run on.
    pub os: String,

    // Config defines the execution parameters which should be used as a base when running a container using the image.
    pub config: Option<ImageConfig>,

    // RootFS references the layer content addresses used by the image.
    pub rootfs: RootFs,

    // History describes the history of each layer.
    pub history: Option<Vec<History>>,
}

mod tests {
    use std::collections::HashMap;

    use chrono::DateTime;
    use serde_json::json;

    use crate::spec::{
        config::{History, Image, ImageConfig, RootFs},
        digest::{Algorithm, Digest},
    };

    #[test]
    fn test_deserialize_image() {
        const IMAGE_JSON: &str = r#"
		{
			"created": "2015-10-31T22:22:56.015925234Z",
			"author": "Alyssa P. Hacker <alyspdev@example.com>",
			"architecture": "amd64",
			"os": "linux",
			"config": {
				"User": "alice",
				"ExposedPorts": {
					"8080/tcp": {}
				},
				"Env": [
					"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
					"FOO=oci_is_a",
					"BAR=well_written_spec"
				],
				"Entrypoint": [
					"/bin/my-app-binary"
				],
				"Cmd": [
					"--foreground",
					"--config",
					"/etc/my-app.d/default.cfg"
				],
				"Volumes": {
					"/var/job-result-data": {},
					"/var/log/my-app-logs": {}
				},
				"WorkingDir": "/home/alice",
				"Labels": {
					"com.example.project.git.url": "https://example.com/project.git",
					"com.example.project.git.commit": "45a939b2999782a3f005621a8d0f29aa387e1d6b"
				}
			},
			"rootfs": {
			  "diff_ids": [
				"sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
				"sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef"
			  ],
			  "type": "layers"
			},
			"history": [
			  {
				"created": "2015-10-31T22:22:54.690851953Z",
				"created_by": "/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /"
			  },
			  {
				"created": "2015-10-31T22:22:55.613815829Z",
				"created_by": "/bin/sh -c #(nop) CMD [\"sh\"]",
				"empty_layer": true
			  }
			]
		}"#;
        let image: Image = serde_json::from_str(IMAGE_JSON).unwrap();
        let mut exposed_ports = HashMap::new();
        exposed_ports.insert(String::from("8080/tcp"), json!({}));

        let mut volumes = HashMap::new();
        volumes.insert(String::from("/var/job-result-data"), json!({}));
        volumes.insert(String::from("/var/log/my-app-logs"), json!({}));

        let mut labels = HashMap::new();
        labels.insert(
            String::from("com.example.project.git.url"),
            String::from("https://example.com/project.git"),
        );
        labels.insert(
            String::from("com.example.project.git.commit"),
            String::from("45a939b2999782a3f005621a8d0f29aa387e1d6b"),
        );

        let config = ImageConfig {
            user: Some(String::from("alice")),
            exposed_ports: Some(exposed_ports),
            env: Some(vec![
                String::from("PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"),
                String::from("FOO=oci_is_a"),
                String::from("BAR=well_written_spec"),
            ]),
            entrypoint: Some(vec![String::from("/bin/my-app-binary")]),
            cmd: Some(vec![
                String::from("--foreground"),
                String::from("--config"),
                String::from("/etc/my-app.d/default.cfg"),
            ]),
            volumes: Some(volumes),
            working_dir: Some(String::from("/home/alice")),
            labels: Some(labels),
            stop_signal: None,
        };
        let rootfs = RootFs {
            typ: String::from("layers"),
            diff_ids: vec![
                Digest {
                    algorithm: Algorithm::Sha256,
                    encoded: String::from(
                        "c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
                    ),
                },
                Digest {
                    algorithm: Algorithm::Sha256,
                    encoded: String::from(
                        "5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef",
                    ),
                },
            ],
        };

        let history1 = History {
			author: None,
			created: Some(DateTime::parse_from_rfc3339("2015-10-31T22:22:54.690851953Z").unwrap()),
			comment: None,
			created_by: Some(String::from("/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /")),
			empty_layer: None,
		};

        let history2 = History {
            author: None,
            created: Some(DateTime::parse_from_rfc3339("2015-10-31T22:22:55.613815829Z").unwrap()),
            comment: None,
            created_by: Some(String::from("/bin/sh -c #(nop) CMD [\"sh\"]")),
            empty_layer: Some(true),
        };

        let histories = vec![history1, history2];
        let expected = Image {
            created: Some(DateTime::parse_from_rfc3339("2015-10-31T22:22:56.015925234Z").unwrap()),
            author: Some(String::from("Alyssa P. Hacker <alyspdev@example.com>")),
            architecture: String::from("amd64"),
            os: String::from("linux"),
            config: Some(config),
            rootfs,
            history: Some(histories),
        };

        assert_eq!(expected, image);
    }
}
