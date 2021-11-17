# OCI extractor

## A tool to extract rootfs from an OCI image

### Usage:
```shell
skopeo copy docker://alpine:latest oci:alpine
./oci-extractor unpack --image alpine alpine_rootfs
```

