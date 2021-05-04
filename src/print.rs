use crate::count::Counts;
use crate::language::Language;
use prettytable::{format, Cell, Row, Table};
use regex::Regex;
use std::collections::HashMap;

pub fn grouped_by_language(
    counts: &HashMap<Language, Counts>,
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
    titles.push(Cell::new("Language").style_spec("b"));
    titles.push(Cell::new("Files").style_spec("b"));
    titles.push(Cell::new("Tokens").style_spec("b"));
    kinds
        .iter()
        .for_each(|kind| titles.push(Cell::new(kind).style_spec("b")));
    kind_patterns
        .iter()
        .for_each(|kind_pat| titles.push(Cell::new(&kind_pat.to_string()).style_spec("b")));
    queries
        .iter()
        .for_each(|query| titles.push(Cell::new(query).style_spec("b")));
    table.set_titles(Row::new(titles));

    counts
        .iter()
        .map(|(lang, count)| {
            let mut cols =
                Vec::with_capacity(3 + kinds.len() + kind_patterns.len() + queries.len());

            // Language
            cols.push(Cell::new(&lang.to_string()).style_spec("li"));
            // number of files
            cols.push(Cell::new(&count.nfiles.to_string()).style_spec("r"));
            // number of tokens
            cols.push(Cell::new(&count.ntokens.to_string()).style_spec("r"));
            // number of nodes for a specific kind
            count
                .nkinds
                .iter()
                .for_each(|n| cols.push(Cell::new(&n.to_string()).style_spec("r")));
            // number of nodes for a specific pattern
            count
                .nkind_patterns
                .iter()
                .for_each(|n| cols.push(Cell::new(&n.to_string()).style_spec("r")));
            // number of nodes for a specific query
            queries.iter().for_each(|query| {
                cols.push(
                    Cell::new(&count.nqueries.get(query).unwrap_or(&0).to_string()).style_spec("r"),
                )
            });
            cols
        })
        .for_each(|row| {
            table.add_row(Row::new(row));
        });

    table.printstd();
}
