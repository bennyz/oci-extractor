use clap::{App, Arg, SubCommand};
use umoci_rs::unpacker::Unpacker;

fn main() {
    let matches = App::new("umoci-rs")
        .version("0.1")
        .author("Benny Zlotnik <bzlotnik@protonmail.com>")
        .about("umoci-rs currently unpacks OCI images")
        .subcommand(
            SubCommand::with_name("unpack")
                .about("unpacks OCI image")
                .arg(
                    Arg::with_name("image")
                        .long("image")
                        .required(true)
                        .takes_value(true)
                        .help("Image name to unapck in the format: <image-path>[:<tag>]"),
                )
                .arg(
                    Arg::with_name("destination")
                        .takes_value(true)
                        .required(true)
                        .help("rootfs target dir"),
                ),
        )
        .get_matches();

    if let Some(unpack) = matches.subcommand_matches("unpack") {
        // Use a struct like normal person
        let image = unpack.value_of("image").unwrap();
        let destination = unpack.value_of("destination").unwrap();

        let u = Unpacker::new(
            String::from(image),
            String::from(destination),
        );
        u.unpack();
    }
}
