use crate::judge::Verdict;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    name: String,
    group: String,
    url: String,
    interactive: bool,
    memory_limit: usize, // mb
    time_limit: usize,   // ms
    tests: Vec<Test>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Test {
    input: String,
    output: String,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Problem {
    pub title: String,
    pub url: String,
    pub memory_limit: usize,
    pub time_limit: usize,
}

impl Test {
    pub fn get_verdict(&self) -> Verdict {
        Verdict {
            answer: self.output.clone(),
            input: self.input.clone(),
            output: "".into(),
            stderr: "".into(),
            memory: 0.0,
            time: 0.0,
            status_id: 0,
            status: "NA".into(),
        }
    }
}

impl Info {
    pub fn get_problem(&self) -> Problem {
        Problem {
            title: self.name.clone(),
            memory_limit: self.memory_limit,
            time_limit: self.time_limit,
            url: self.url.clone(),
        }
    }

    pub fn get_verdicts(&self) -> Vec<Verdict> {
        self.tests.iter().map(|x| x.get_verdict()).collect()
    }
}
