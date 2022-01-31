pub mod cli;

use clap::StructOpt;
use cli::Args;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SearchResult {
    value: String,
    start: i32,
    end: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ResultObject {
    regex: String,
    lines: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RegexMap {
    matches: HashMap<String, ResultObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CaptureLocation {
    start: usize,
    end: usize,
    line: String,
}

impl RegexMap {
    fn new(regexs: &[String]) -> Self {
        let mut m: HashMap<String, ResultObject> = HashMap::<String, ResultObject>::new();
        for mat in regexs {
            m.insert(
                mat.to_owned(),
                ResultObject {
                    regex: mat.to_owned(),
                    lines: Vec::new(),
                },
            );
        }
        RegexMap { matches: m }
    }

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
    let mut matches = match_multiregex(regex_items.as_slice(), &input_text).unwrap();
    if !args.output.is_none() {
        fs::write(args.output.unwrap(), matches.to_json(false)).unwrap();
    } else {
        print!("Matches: {:?}", matches.to_json(true));
    }
}

fn get_matched_items(r: Regex, s: &str) -> Option<Vec<CaptureLocation>> {
    let captures: Vec<CaptureLocation> = r
        .captures_iter(s)
        .flat_map(|mat| {
            mat.iter()
                .map(|m| {
                    let ma = m.unwrap().to_owned();
                    CaptureLocation {
                        start: ma.start(),
                        end: ma.end(),
                        line: m.unwrap().as_str().to_owned(),
                    }
                })
                .collect::<Vec<CaptureLocation>>()
        })
        .collect::<Vec<CaptureLocation>>();
    Some(captures)
}

fn match_multiregex(regex_set: &[String], string: &str) -> Option<RegexMap> {
    // let set = RegexSet::new(regex_set).unwrap();
    let regexi = regex_set
        .clone()
        .into_iter()
        .map(|n| RegexBuilder::new(n).build().unwrap())
        .collect::<Vec<Regex>>();
    // let found = set.matches(string).to_owned();
    let mut final_map = RegexMap::new(&regex_set);
    let t: Vec<&Regex> = regexi.iter().collect();
    let v = t
        .into_par_iter()
        .map(|r| {
            let mut fin_map = final_map.clone();
            let v: Vec<CaptureLocation> = get_matched_items(r.clone(), string).unwrap();
            for t in v.iter() {
                fin_map.matches.get_mut(r.as_str()).unwrap().lines.insert(
                    0,
                    SearchResult {
                        start: t.start as i32,
                        end: t.end as i32,
                        value: t.line.clone(),
                    },
                );
            }
            fin_map
        })
        .collect::<Vec<RegexMap>>();

    for map in v {
        for key in map.matches.keys() {
            final_map
                .matches
                .insert(key.clone(), map.matches[key].clone());
        }
    }
    return Some(final_map);
}
