use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    schema_version: u16,
    pub name: String,
    pub tag: String,
    pub architecture: String,
    #[serde(rename = "fsLayers")]
    pub fs_layers: Vec<FSLayer>,
    pub history: Vec<V1Compatability>,
    pub signatures: Vec<Signature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    header: serde_json::Value,
    signature: String,
    protected: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct V1Compatability {
    #[serde(rename = "v1Compatibility")]
    v1_compatability: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FSLayer {
    #[serde(rename = "blobSum")]
    pub blob_sum: String,
}
