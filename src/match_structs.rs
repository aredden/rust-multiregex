use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub value: String,
    pub start: i32,
    pub end: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultObject {
    pub regex: String,
    pub lines: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegexMap {
    pub matches: HashMap<String, ResultObject>,
}

impl RegexMap {
    pub fn new(regexs: &[String]) -> Self {
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
