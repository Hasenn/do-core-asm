use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "do core assembler",
    about = "An assembler for the do-core fantasy CPU architecture"
)]
pub struct Opt {
    #[structopt(short,long, parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short,long, parse(from_os_str))]
    pub output: Option<PathBuf>,
}
