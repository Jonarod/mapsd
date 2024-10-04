use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mapsd", about = "Find & Replace text in multiple files using an explicit CSV of Before/After pairs.")]
pub struct Opt {
    /// Files to process (glob pattern)
    #[structopt(name = "FILES")]
    pub files: String,

    /// Path to the CSV file containing key/value pairs
    #[structopt(short, long, default_value = "map.csv")]
    pub map: String,

    /// CSV delimiter
    #[structopt(short, long, default_value = ",")]
    pub delimiter: String,

    /// CSV has headers
    #[structopt(long)]
    pub has_headers: bool,

    /// Prefix to use for the resulting copied file
    #[structopt(short, long, default_value = "replaced.")]
    pub prefix: String,

    
    /// Replace files in-place (USE WITH CAUTION)
    #[structopt(long = "DANGEROUSLY-REPLACE-INPLACE")]
    pub inplace: bool,

    /// Suppress output
    #[structopt(long)]
    pub silent: bool,
}
