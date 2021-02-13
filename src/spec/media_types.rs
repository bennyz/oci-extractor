use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// [image-spec]: https://github.com/opencontainers/image-spec/blob/v1.0.1/media-types.md
pub enum MediaType {
    ContentDescriptor,
    OciLayout,
    ImageIndex,
    ImageManifest,
    ImageConfig,
    ImageLayerTar,
    ImageLayerTarGzip,
    ImageLayerZstd,
    ImageLayerNondistributableTar,
    ImageLayerNondistributableTarGzip,
    ImageLayerNonDistributableZstd,
}

impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            MediaType::ContentDescriptor => "application/vnd.oci.descriptor.v1+json",
            MediaType::OciLayout => "application/vnd.oci.layout.header.v1+json",
            MediaType::ImageIndex => "application/vnd.oci.image.index.v1+json",
            MediaType::ImageManifest => "application/vnd.oci.image.manifest.v1+json",
            MediaType::ImageConfig => "application/vnd.oci.image.config.v1+json",
            MediaType::ImageLayerTar => "application/vnd.oci.image.layer.v1.tar",
            MediaType::ImageLayerTarGzip => "application/vnd.oci.image.layer.v1.tar+gzip",
            MediaType::ImageLayerZstd => "application/vnd.oci.image.layer.v1.tar+zstd",
            MediaType::ImageLayerNondistributableTar => {
                "application/vnd.oci.image.layer.nondistributable.v1.tar"
            }
            MediaType::ImageLayerNondistributableTarGzip => {
                "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip"
            }
            MediaType::ImageLayerNonDistributableZstd => {
                "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd"
            }
        })
    }
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "application/vnd.oci.descriptor.v1+json" => MediaType::ContentDescriptor,
            "application/vnd.oci.layout.header.v1+json" => MediaType::OciLayout,
            "application/vnd.oci.image.index.v1+json" => MediaType::ImageIndex,
            "application/vnd.oci.image.manifest.v1+json" => MediaType::ImageManifest,
            "application/vnd.oci.image.config.v1+json" => MediaType::ImageConfig,
            "application/vnd.oci.image.layer.v1.tar" => MediaType::ImageLayerTar,
            "application/vnd.oci.image.layer.v1.tar+gzip" => MediaType::ImageLayerTarGzip,
            "application/vnd.oci.image.layer.v1.tar+zstd" => MediaType::ImageLayerZstd,
            "application/vnd.oci.image.layer.nondistributable.v1.tar" => {
                MediaType::ImageLayerNondistributableTar
            }
            "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip" => {
                MediaType::ImageLayerNondistributableTarGzip
            }
            "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd" => {
                MediaType::ImageLayerNonDistributableZstd
            }
            _ => panic!("Invalid media type!"),
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn deserialize_media_type() {
        let media_type: MediaType =
            serde_json::from_str(r#""application/vnd.oci.descriptor.v1+json""#).unwrap();
        assert_eq!(media_type, MediaType::ContentDescriptor);
    }
}
