pub mod cli;
mod match_structs;

use clap::StructOpt;
use cli::{Args, FlagArgs};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::{Regex, RegexBuilder};
use std::{fs::{read_to_string, write}, path::Path};
use match_structs::{RegexMap, SearchResult};


fn build_regex(regex_set: Vec::<String>, flags: &FlagArgs) -> Vec<Regex> {
    regex_set
    .clone()
    .into_iter()
    .map(|n| {
        RegexBuilder::new(n.as_str())
            .multi_line(flags.multiline)
            .case_insensitive(flags.case_insensitive)
            .unicode(flags.unicode)
            .build()
            .unwrap()
    })
    .collect::<Vec<Regex>>()
}

fn main() {
    let args = Args::parse();
    assert!(!args.input.is_empty(), "'input' argument is empty");
    assert!(
        !args.regex_file.is_empty(),
        "'regex_file' argument is empty"
    );

    let input_text = read_to_string(&args.input).expect(
        format!(
            "Something went wrong reading the file at \"{:?}\"",
            &args.input
        )
        .as_str(),
    );
    let mut regex_items: Vec<String> = Vec::new();
    if Path::new(&args.regex_file).exists() {
        let content = read_to_string(&args.regex_file).expect(
            format!(
                "Something went wrong reading the file at \"{:?}\"",
                &args.regex_file
            )
            .as_str(),
        );
        content
            .lines()
            .for_each(|reg| regex_items.push(reg.to_string()));
    } else {
        regex_items = args
            .regex_file
            .split("::")
            .map(|reg| reg.to_string())
            .collect();
    }


    let mut matches = match_multiregex(regex_items.as_slice(), &input_text, args.flags).unwrap();
    if !args.output.is_none() {
        write(args.output.unwrap(), matches.to_json(args.pretty)).unwrap();
    } else {
        print!("Matches: {:?}", matches.to_json(true));
    }
}

fn get_matched_items(r: Regex, s: &str) -> Option<Vec<SearchResult>> {
    let captures: Vec<SearchResult> = r
        .captures_iter(s)
        .flat_map(|mat| {
            mat.iter()
                .map(|m| {
                    let ma = m.unwrap().to_owned();
                    SearchResult {
                        start: ma.start() as i32,
                        end: ma.end() as i32,
                        value: m.unwrap().as_str().to_owned(),
                    }
                })
                .collect::<Vec<SearchResult>>()
        })
        .collect::<Vec<SearchResult>>();
    Some(captures)
}

fn match_multiregex(regex_set: &[String], string: &str, flags: FlagArgs) -> Option<RegexMap> {
    let regexi = build_regex(regex_set.to_vec(), &flags);

    let mut final_map = RegexMap::new(&regex_set);
    let v = regexi
        .into_par_iter()
        .map(|r| {
            let mut fin_map = final_map.clone();
            let v: Vec<SearchResult> = get_matched_items(r.clone(), string).unwrap();
            fin_map
                .matches
                .get_mut(&r.to_string())
                .unwrap()
                .lines
                .extend(v);
            fin_map
        })
        .collect::<Vec<RegexMap>>();

    for map in v.iter() {
        final_map.matches.iter_mut().for_each(|(k, v)| {
            v.lines.extend(map.matches.get(k).unwrap().lines.to_owned());
        })
    }
    return Some(final_map);
}