use colored::*;
use ignore::Walk;
use std::{collections::HashMap, fs};

use crate::{
    args::{Precision, RowDisplay},
    format::{TableRow, COLORS_BARS},
    lang::to_readable,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Measure {
    #[default]
    Files,
    KBytes,
}

#[derive(Debug)]
pub struct ProjectStats {
    path: String,
    map: HashMap<String, u64>,
    total: u64,
    precision: Precision,
    measure: Measure,
}

impl ProjectStats {
    pub fn new(path: String) -> Self {
        ProjectStats {
            path,
            map: HashMap::new(),
            total: 0,
            precision: Precision::Single,
            measure: Default::default(),
        }
    }

    pub fn process_by_file_count(&mut self) {
        for entry in Walk::new(&self.path).filter_map(|e| e.ok()) {
            if let Some(extension) = entry.path().extension() {
                let extension = extension.to_str().unwrap();
                let extension = to_readable(extension);

                if self.map.contains_key(&extension) {
                    self.map.insert(
                        extension.to_string(),
                        *self.map.get(&extension).unwrap() + 1,
                    );
                } else {
                    self.map.insert(extension.to_string(), 1);
                };
            }
        }

        self.total = self.map.values().sum::<u64>();
        self.precision = if self.total_files() > 1000 {
            Precision::Double
        } else {
            Precision::Single
        };
        self.measure = Measure::Files;
    }

    pub fn process_by_file_size(&mut self) {
        for entry in Walk::new(&self.path).filter_map(|e| e.ok()) {
            if let Some(extension) = entry.path().extension() {
                let extension = extension.to_str().unwrap();
                let extension = to_readable(extension);

                if let Ok(metadata) = fs::metadata(entry.path()) {
                    let kbytes = metadata.len() / 1000;
                    if self.map.contains_key(&extension) {
                        *self.map.get_mut(&extension).unwrap() += kbytes;
                    } else {
                        self.map.insert(extension.to_string(), kbytes);
                    }
                }
            }
        }

        self.total = self.map.values().sum::<u64>();
        self.precision = Precision::Double;
        self.measure = Measure::KBytes;
    }

    pub fn to_table(&self, is_compact_output: bool, output: RowDisplay) -> Vec<TableRow> {
        let mut acc_percentage = 0.0;
        let mut table: Vec<TableRow> = Vec::new();

        for (index, (ext, count)) in self.to_sorted_list().iter().rev().enumerate() {
            let percentage = (**count as f64 / self.total_files() as f64) * 100.0;
            let has_shown_enough = is_compact_output && percentage < 0.5;
            if has_shown_enough {
                break;
            }

            let row = TableRow {
                lang: ext,
                percentage,
                value: **count as u32,
                color: COLORS_BARS[index % COLORS_BARS.len()],
                precision: self.precision(),
                output,
                measure: self.measure.to_owned(),
            };

            acc_percentage += percentage;
            table.push(row);
        }

        if is_compact_output && !(99.9..=100.1).contains(&acc_percentage) {
            // print 'Other' row
            let remaining_percentage = 100.0 - acc_percentage;
            let remaining_count =
                (self.total_files() as f64 * remaining_percentage / 100.0).round() as u32;

            let other_row = TableRow {
                lang: "Other",
                percentage: remaining_percentage,
                value: remaining_count,
                color: Color::White,
                precision: self.precision(),
                output,
                measure: self.measure.to_owned(),
            };
            table.push(other_row);
        }

        table
    }

    pub fn total_files(&self) -> u64 {
        self.total
    }

    pub fn precision(&self) -> Precision {
        self.precision
    }

    pub fn to_sorted_list(&self) -> Vec<(&String, &u64)> {
        let mut sorted_entries: Vec<(&String, &u64)> = self.map.iter().collect();
        sorted_entries.sort_by_key(|&(_, value)| *value);

        sorted_entries
    }
}
