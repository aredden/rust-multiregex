use clap::Parser;
/// Multiple regex matcher
#[derive(Parser, Debug)]
#[clap(author="Alex Redden", version="0.0.1", about, long_about = None, name="process")]
pub struct Args {
    /// Input text file.
    #[clap(short = 'i', long)]
    pub input: String,

    /// Output text file, ignore for stdout.
    #[clap(short = 'o', long)]
    pub output: Option<String>,

    /// Regext file containing each regex on a new line.
    #[clap(short = 'r', long)]
    pub regex_file: String,
}
