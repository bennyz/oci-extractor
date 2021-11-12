use clap::Parser;
use oci_extractor::copy::Copy;
use oci_extractor::unpacker::Unpacker;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Unpack(UnpackCmd),
    Copy(CopyCmd),
}

#[derive(Parser)]
struct UnpackCmd {
    #[clap(long)]
    image: String,
    destination: String,
}

#[derive(Parser)]
struct CopyCmd {
    #[clap(long)]
    image: String,
    destination: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Unpack(u) => {
            let unpacker = Unpacker::new(u.image, u.destination);
            unpacker.unpack();
        }
        SubCommand::Copy(c) => {
            println!("image {} dest {}", c.image, c.destination);
            let copy = Copy::new(c.image, c.destination);
            copy.run().await;
        }
    }
}
