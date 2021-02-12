

/// the file name of oci image layout file
pub const IMAGE_LAYOUT: &str = "oci-layout";

/// the version of Image Layout file
pub const IMAGE_LAYOUT_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImageLayout {
	image_layout_version: String,
}
