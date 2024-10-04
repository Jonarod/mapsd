use mapsd::{run, arguments::Opt};
use structopt::StructOpt;
use std::io;
use env_logger::Builder;
use log::LevelFilter;

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

