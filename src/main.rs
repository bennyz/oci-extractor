use clap::{AppSettings, Parser};
use umoci_rs::unpacker::Unpacker;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Unpack(Unpack),
}

#[derive(Parser)]
struct Unpack {
    #[clap(long)]
    image: String,
    destination: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Unpack(u) => {
            let unpacker = Unpacker::new(u.image, u.destination);
            unpacker.unpack();
        }
    }
}
