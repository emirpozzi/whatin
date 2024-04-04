use colored::*;

use crate::args::{Precision, RowDisplay};
use crate::dto::WhatInResult;
use crate::stats::Measure;

pub const COLORS_BARS: [Color; 6] = [
    Color::Blue,
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Magenta,
    Color::Cyan,
];

/// In a TableRow the output is always printed by sequentially printing each row
#[derive(Debug)]
pub struct TableRow<'a> {
    pub lang: &'a str,
    pub percentage: f64,
    pub value: u32,
    pub color: Color,
    pub precision: Precision,
    pub output: RowDisplay,
    pub measure: Measure,
}

impl<'a> TableRow<'a> {
    pub fn to_representation(&self) -> WhatInResult {
        match self.measure {
            Measure::Files => WhatInResult::AsCount {
                language: self.lang.to_string(),
                percentage: self.percentage.to_string(),
                number_of_files: self.value.to_string(),
            },
            Measure::KBytes => WhatInResult::InBytes {
                language: self.lang.to_string(),
                percentage: self.percentage.to_string(),
                kbytes: self.value.to_string(),
            },
        }
    }
}

impl<'a> std::fmt::Display for TableRow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self.measure {
            Measure::Files => {
                if self.value == 1 {
                    "file"
                } else {
                    "files"
                }
            }
            Measure::KBytes => "KB",
        };

        let bar = "█"
            .repeat(self.percentage.round() as usize)
            .color(self.color);

        match (self.precision, self.output) {
            (Precision::Double, RowDisplay::Csv) => {
                write!(f, "{},{:.2},{}", self.lang, self.percentage, self.value,)
            }
            (Precision::Single, RowDisplay::Csv) => {
                write!(f, "{},{:.1},{}", self.lang, self.percentage, self.value,)
            }
            (Precision::Double, RowDisplay::Table) => write!(
                f,
                "{:<20} | {:>5.2}% | {:>9} {:<5} {}",
                self.lang.cyan().bold(),
                self.percentage,
                self.value,
                label,
                bar
            ),
            (Precision::Single, RowDisplay::Table) => write!(
                f,
                "{:<20} | {:>5.1}% | {:>9} {:<5} {}",
                self.lang.cyan().bold(),
                self.percentage,
                self.value,
                label,
                bar
            ),
            (Precision::Double, RowDisplay::Pretty) => {
                write!(
                    f,
                    "{} - {} {}\n{} {:.2}%",
                    self.lang, self.value, label, bar, self.percentage
                )
            }
            (Precision::Single, RowDisplay::Pretty) => {
                write!(
                    f,
                    "{} - {} {}\n{} {:.1}%",
                    self.lang, self.value, label, bar, self.percentage
                )
            }
            (_, RowDisplay::OneLine) => {
                let circle = "⬤".color(self.color);
                write!(f, "{}  {}      ", circle, self.lang)
            }
        }
    }
}
