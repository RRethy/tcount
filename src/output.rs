use crate::count::Counts;
use prettytable::{format, Cell, Row, Table};
use regex::Regex;
use std::fmt::Display;
use std::format;

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

pub fn table(
    counts: &Vec<(impl Display, Counts)>,
    kinds: &Vec<String>,
    kind_patterns: &Vec<Regex>,
    queries: &Vec<String>,
) {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .borders('│')
        .separators(
            &[format::LinePosition::Top],
            format::LineSeparator::new('─', '─', '╭', '╮'),
        )
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('─', '─', '│', '│'),
        )
        .separators(
            &[format::LinePosition::Bottom],
            format::LineSeparator::new('─', '─', '╰', '╯'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    let mut titles = Vec::with_capacity(3 + kinds.len() + kind_patterns.len() + queries.len());
    titles.push(title_cell(""));
    titles.push(title_cell("Files"));
    titles.push(title_cell("Tokens"));
    kinds
        .iter()
        .for_each(|kind| titles.push(title_cell(&format!("Kind({})", kind))));
    kind_patterns.iter().for_each(|kind_pat| {
        titles.push(title_cell(&format!("Pattern({})", kind_pat.to_string())))
    });
    queries
        .iter()
        .for_each(|query| titles.push(title_cell(&format!("Query({})", query)).style_spec("b")));
    table.set_titles(Row::new(titles));

    counts
        .iter()
        .map(|(lang, count)| {
            let mut cols =
                Vec::with_capacity(3 + kinds.len() + kind_patterns.len() + queries.len());

            // Language
            cols.push(label_cell(&lang.to_string()));
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
            queries
                .iter()
                .for_each(|query| cols.push(count_cell(*count.nqueries.get(query).unwrap_or(&0))));
            cols
        })
        .for_each(|row| {
            table.add_row(Row::new(row));
        });

    table.printstd();
}
