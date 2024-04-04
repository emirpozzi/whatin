use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WhatInResult {
    AsCount {
        language: String,
        percentage: String,
        number_of_files: String,
    },
    InBytes {
        language: String,
        percentage: String,
        kbytes: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregatedResult {
    data: Vec<WhatInResult>,
}

impl AggregatedResult {
    pub fn new(data: Vec<WhatInResult>) -> Self {
        AggregatedResult { data }
    }
}
