use clap::Parser;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowDisplay {
    Table,
    Csv,
    Pretty,
    OneLine,
}

/// Simple program to have an overview of a project by programming language.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path where to look for
    #[arg(short, long, default_value_t = String::from("."))]
    pub path: String,

    /// Change the type of output
    #[clap(short, long, default_value_t, value_enum)]
    pub style: Layout,

    /// Print the full result
    #[arg(short, long, default_value_t = false)]
    pub full: bool,

    /// Calculate by number of files or Kbytes
    #[arg(short, long, default_value_t, value_enum)]
    pub unit: CalculateBy,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
pub enum Layout {
    #[default]
    Pretty,
    Table,
    OneLine,
    Csv,
    Json,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
pub enum CalculateBy {
    #[default]
    File,
    Kb,
}
