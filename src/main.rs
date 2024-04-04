use crate::args::Args;
use args::RowDisplay;
use clap::Parser;
use dto::AggregatedResult;
use graphs::{Graph, OneLineGraph};
use log::LevelFilter;
use spinners::{Spinner, Spinners};
use stats::ProjectStats;

mod args;
mod dto;
mod format;
mod graphs;
mod lang;
mod stats;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Off)
        .init();

    let args = Args::parse();
    let is_compact_output = !args.full;

    let mut spinner = Spinner::new(Spinners::Aesthetic, "Just a sec...".into());
    let mut stats = ProjectStats::new(args.path);
    match args.unit {
        args::CalculateBy::File => stats.process_by_file_count(),
        args::CalculateBy::Kb => stats.process_by_file_size(),
    }
    spinner.stop_with_newline();

    let row_display = match args.style {
        args::Layout::Pretty => RowDisplay::Pretty,
        args::Layout::Table => RowDisplay::Table,
        args::Layout::OneLine => RowDisplay::OneLine,
        args::Layout::Csv | args::Layout::Json => RowDisplay::Csv,
    };

    match args.style {
        args::Layout::Pretty | args::Layout::Table | args::Layout::Csv => {
            for row in stats.to_table(is_compact_output, row_display).iter() {
                println!("{row}");
            }
        }
        args::Layout::Json => {
            let table = stats
                .to_table(is_compact_output, RowDisplay::Csv)
                .iter()
                .map(|value| value.to_representation())
                .collect();

            let dto = AggregatedResult::new(table);
            println!("{}", serde_json::to_string(&dto).unwrap());
        }
        args::Layout::OneLine => {
            let table = stats.to_table(is_compact_output, row_display);
            let graph = OneLineGraph::new(table);

            graph.print();
        }
    };
}
