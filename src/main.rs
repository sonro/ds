use clap::Clap;

fn main() {
    let opts = Opts::parse();

    for path in opts.paths {
        println!("{}", path);
    }
}

/// Destroy - removes files and directories
#[derive(Clap)]
#[clap(version = "0.1", author = "Christopher Morton <sonro@gmx.com>")]
struct Opts {
    /// List of file paths to remove
    paths: Vec<String>,
}
