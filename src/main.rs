use env_logger::Builder;
use log::LevelFilter;
use mapsd::{arguments::Opt, run};
use std::io;
use structopt::StructOpt;

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    // Initialize logger
    let mut builder = Builder::from_default_env();
    if opt.silent {
        builder.filter_level(LevelFilter::Error);
    } else {
        builder.filter_level(LevelFilter::Info);
    }
    builder.init();

    run(&opt)
}
