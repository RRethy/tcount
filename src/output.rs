use crate::count::Counts;
use crate::language::Language;
use crate::query::{Query, QueryKind};
use prettytable::{format, Cell, Row, Table};
use regex::Regex;
use std::fmt::Display;
use std::format;
use std::str::FromStr;

#[derive(Debug)]
pub enum Format {
    Table,
    Csv,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "table" => Ok(Format::Table),
            "csv" => Ok(Format::Csv),
            _ => Err(format!("\"{}\" is not supported. Use one of table|csv", s)),
        }
    }
}

pub fn format_builder() -> format::FormatBuilder {
    format::FormatBuilder::new()
        .separators(
            &[format::LinePosition::Top],
            format::LineSeparator::new('─', '─', '─', '─'),
        )
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('─', '─', '│', '│'),
        )
        .separators(
            &[format::LinePosition::Bottom],
            format::LineSeparator::new('─', '─', '─', '─'),
        )
        .padding(1, 1)
}

#[inline]
fn title_cell(content: &str) -> Cell {
    Cell::new(content).style_spec("b")
}
#[inline]
fn label_cell(label: &str) -> Cell {
    Cell::new(label).style_spec("li")
}
#[inline]
fn count_cell(count: u64) -> Cell {
    Cell::new(&count.to_string()).style_spec("r")
}

#[inline]
fn generic_cell(s: impl Display) -> Cell {
    Cell::new(&s.to_string()).style_spec("l")
}

pub fn print(
    format: &Format,
    counts: Vec<(String, Counts)>,
    totals: Option<Counts>,
    kinds: &Vec<String>,
    kind_patterns: &Vec<Regex>,
    queries: &Vec<Query>,
) {
    let mut table = Table::new();
    table.set_format(format_builder().build());

    let mut titles = Vec::with_capacity(3 + kinds.len() + kind_patterns.len() + queries.len());
    titles.push(title_cell("Group"));
    titles.push(title_cell("Files"));
    titles.push(title_cell("Tokens"));
    kinds
        .iter()
        .for_each(|kind| titles.push(title_cell(&format!("Kind({})", kind))));
    kind_patterns
        .iter()
        .for_each(|kind_pat| titles.push(title_cell(&format!("Pattern({})", kind_pat))));
    queries.iter().for_each(|query| match &query.kind {
        QueryKind::Match => titles.push(title_cell(&format!("Query({})", query.name))),
        QueryKind::Captures(names) => names.iter().for_each(|name| {
            titles.push(title_cell(&format!("Query({}@{})", query.name, name)));
        }),
    });
    table.set_titles(Row::new(titles));

    counts
        .iter()
        .chain(
            {
                if let Some(totals) = totals {
                    vec![(String::from("TOTALS"), totals)]
                } else {
                    vec![]
                }
            }
            .iter(),
        )
        .map(|(label, count)| {
            let mut cols =
                Vec::with_capacity(3 + kinds.len() + kind_patterns.len() + queries.len());

            // Language
            cols.push(label_cell(&label.to_string()));
            // number of files
            cols.push(count_cell(count.nfiles));
            // number of tokens
            cols.push(count_cell(count.ntokens));
            // number of nodes for a specific kind
            count.nkinds.iter().for_each(|n| cols.push(count_cell(*n)));
            // number of nodes for a specific pattern
            count
                .nkind_patterns
                .iter()
                .for_each(|n| cols.push(count_cell(*n)));
            // number of nodes for a specific query
            count
                .nqueries
                .iter()
                .for_each(|n| cols.push(count_cell(*n)));
            cols
        })
        .for_each(|row| {
            table.add_row(Row::new(row));
        });

    match format {
        Format::Table => {
            table.printstd();
        }
        Format::Csv => match table.to_csv(std::io::stdout()) {
            Ok(_) => {}
            Err(err) => eprintln!("{}", err),
        },
    }
}

pub fn print_languages(langs: Vec<(&Language, Vec<String>, &Vec<String>)>) {
    let mut table = Table::new();
    table.set_format(format_builder().build());

    let titles = vec![
        title_cell("Language"),
        title_cell("Extensions"),
        title_cell("Query Dir Name"),
    ];
    table.set_titles(Row::new(titles));

    langs.into_iter().for_each(|(lang, exts, dirs)| {
        let cols = vec![
            label_cell(&lang.to_string()),
            generic_cell(exts.join(",")),
            generic_cell(dirs.join(",")),
        ];
        table.add_row(Row::new(cols));
    });

    table.printstd();
}
