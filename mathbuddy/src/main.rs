use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();

    let result = if options.file {
        let mut fh = File::open(options.input).unwrap();
        let mut buf = String::new();
        fh.read_to_string(&mut buf).unwrap();
        mathbuddy_lib::convert(&buf)
    } else {
        mathbuddy_lib::convert(&options.input)
    };

    output(&options.output, &result);
}

fn output(destination: &Option<PathBuf>, payload: &str) {
    match destination {
        Some(path) => {
            let mut fh = File::create(path).unwrap();
            fh.write_all(&payload.as_bytes()).unwrap();
        }
        None => println!("{}", payload),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "mathbuddy")]
struct Options {
    /// Output file. STDOUT if not set.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// If this flag is set, the main parameter is interpreted as a file name
    /// rather than a LaTeX formula.
    #[structopt(short, long)]
    file: bool,

    #[structopt(name = "FORMULA | FILE")]
    input: String,
}
