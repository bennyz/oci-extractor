use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Arguments represents the options umoci-rs accepts
struct Arguments {
    /// whether or not to jump
    #[argh(subcommand)]
    command: Command,
}
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
/// Enum represnting available commands
pub enum Command {
    Unpack(UnpackCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "unpack")]
/// Unpacks an OCI image
pub struct UnpackCommand {
    #[argh(option)]
    /// how many x
    x: usize,
}

fn main() {
    println!("Hello, world!");
}
