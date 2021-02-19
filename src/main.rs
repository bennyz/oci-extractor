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
                    Arg::with_name("bundle")
                        .takes_value(true)
                        .required(true)
                        .help("path of the bundle"),
                ),
        )
        .get_matches();

    if let Some(unpack) = matches.subcommand_matches("unpack") {
        // Use the struct like normal
        let image = unpack.value_of("image").unwrap();
        let bundle = unpack.value_of("bundle").unwrap();
        println!("image: {}", image);
        println!("bundle: {}", bundle);
        let u = Unpacker::new(String::from(image), String::from(bundle));
        u.unpack();
    }
}
