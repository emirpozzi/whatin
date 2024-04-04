use crate::format::TableRow;
use colored::*;

/// In a Graph the output requires a different approach to be printed than simply printing each row
pub trait Graph {
    fn print(&self);
}

pub struct OneLineGraph<'a> {
    table: Vec<TableRow<'a>>,
}

impl<'a> OneLineGraph<'a> {
    pub fn new(table: Vec<TableRow<'a>>) -> Self {
        OneLineGraph { table }
    }
}

impl<'a> Graph for OneLineGraph<'a> {
    fn print(&self) {
        for row in &self.table {
            let bar = "█".repeat(row.percentage.round() as usize).color(row.color);
            print!("{}", bar);
        }
        println!();

        let mut count = 0;
        for row in &self.table {
            print!("{row}");
            count += 1;
            if count % 4 == 0 {
                println!();
            };
        }
    }
}
