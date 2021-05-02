use crate::count::Counts;
use crate::language::Language;
use std::collections::BTreeMap;

pub fn grouped_by_language(counts: &BTreeMap<Language, Counts>) {
    println!("{:?}", counts);
}

pub fn grouped_by_file(counts: &Vec<(Language, Counts)>) {
    println!("{:?}", counts);
}
