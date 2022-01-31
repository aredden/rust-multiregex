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
    #[clap(short = 'r', long, required = false, value_name = "FILE", parse(from_str))]
    pub regex_file: String,

    /// Regex flags: 
    /// i = ignore case; default: false
    /// m = multi-line; default: false
    /// u = no-unicode; default: false # Can cause issues if false, will throw errors if using .* in regex.
    /// using '|' as a delimiter.
    #[clap(
        short = 'f',
        long,
        parse(from_str=Args::parse_flags),
        multiple_values = true,
        value_delimiter = '|',
        default_value = "",
        required=false
    )]
    pub flags: FlagArgs,

    /// Boolan value representing whether to prettify the json response.
    #[clap(
        short = 'p',
        long,
        parse(try_from_str),
        required = false,
        default_value = "true"
    )]
    pub pretty: bool,
}

#[derive(Default, Debug)]
pub struct FlagArgs {
    pub multiline: bool,
    pub case_insensitive: bool,
    pub unicode: bool,
}

impl Args {
    pub fn parse_flags(selected: &str) -> FlagArgs {
        let mut flags = FlagArgs::default();
        flags.multiline = false;
        flags.case_insensitive = false;
        flags.unicode = true;
        for opt in selected.split("|") {
            match opt.as_ref() {
                "i" => {
                    flags.case_insensitive = true;
                }
                "m" => {
                    flags.multiline = true;
                }
                "u" => {
                    flags.unicode = false;
                }
                _ => panic!("Invalid value for flag"),
            }
        }
        flags
    }
}
