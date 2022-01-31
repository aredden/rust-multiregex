use clap::{AppSettings, Parser};

/// Multiple regex matcher
#[derive(Parser, Debug)]
#[clap(author="Alex Redden", version="0.0.1", about, long_about = None, name="process")]
#[clap(global_setting(AppSettings::AllowMissingPositional))]
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

    /// 1 is pretty 0 is compact.
    #[clap(short, long, default_value_t = 1)]
    pub pretty: i32,
}
