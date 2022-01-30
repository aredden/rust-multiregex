pub mod cli;

use clap::StructOpt;
use cli::Args;
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    value: String,
    line_no: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultObject {
    regex: String,
    lines: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegexMap {
    matches: Vec<ResultObject>,
}

impl RegexMap {
    pub fn to_json(&mut self, pretty: bool) -> String {
        //return the results as a json string
        if pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

fn main() {
    let args = Args::parse();
    let input_text = fs::read_to_string(args.input).unwrap();
    let regex_items = fs::read_to_string(args.regex_file).unwrap();
    let regex_items: Vec<String> = regex_items
        .lines()
        .map(|r| r.to_owned())
        .collect::<Vec<String>>();
    let matches = match_multiregex(regex_items.as_slice(), &input_text);
    let mut m = match matches {
        Some(val) => val,
        None => panic!("called `Option::unwrap()` on a `None` value"),
    };
    if !args.output.is_none() {
        fs::write(args.output.unwrap(), &m.to_json(true)).unwrap();
    } else {
        print!("Matches: {:?}", m.to_json(true));
    }
}

fn get_matched_items(r: Regex, s: &str) -> Option<Vec<String>> {
    let captures: Vec<String> = r
        .captures_iter(s)
        .map(|mat| {
            mat.iter()
                .map(|m| m.unwrap().to_owned().as_str().to_owned())
                .collect::<Vec<String>>()
                .to_owned()
        })
        .collect::<Vec<Vec<String>>>()
        .iter()
        .flatten()
        .map(|e| e.to_owned())
        .collect::<_>();
    Some(captures)
}

fn match_multiregex(regex_set: &[String], string: &str) -> Option<RegexMap> {
    let set = RegexSet::new(regex_set).unwrap();
    let num_regex = regex_set.len().to_owned();
    let found = set.matches(string).to_owned();
    let result = RegexMap {
        matches: {
            let mut found_matches: Vec<ResultObject> = Vec::new();
            for i in 0..num_regex {
                let mut src_results: Vec<SearchResult> = Vec::new();
                for line in found.iter() {
                    let r = Regex::new(&regex_set[i]).unwrap();
                    if r.is_match(string) && line == i {
                        let v: Vec<String> = get_matched_items(r, string).unwrap();
                        if v.len() > 0 {
                            for v in v.iter() {
                                src_results.push(SearchResult {
                                    line_no: i as i32,
                                    value: v.to_owned(),
                                });
                            }
                        }
                    }
                }
                let obj = ResultObject {
                    regex: regex_set[i].to_owned(),
                    lines: src_results,
                };
                found_matches.append(&mut Vec::from([obj]));
            }
            found_matches
        },
    };
    Some(result)
}