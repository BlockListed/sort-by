use std::{io::{stdin, BufRead, stdout, Write, BufWriter}, env::args};

use regex::Regex;

struct Sortable {
    value: String,
    key: i64,
}

fn main() {
    let input = stdin().lock();

    let sort_by_regex = args().nth(1).unwrap();

    let sort_by = Regex::new(&sort_by_regex).unwrap();

    let mut lines: Vec<_> = input
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|value| {
            let Some(key) = extract_sorting_key(&value, &sort_by) else {
                return None
            };
            Some(Sortable {
                value,
                key: key,
            })
        })
        .collect();

    eprintln!("Acquired lines");

    lines.sort_by(|a, b| {
        // Switched around, because we want descending sorts
        std::cmp::Ord::cmp(&b.key, &a.key)
    });

    eprintln!("Sorted");

    let mut output = BufWriter::new(stdout().lock());
    for i in lines {
        writeln!(&mut output, "{}", i.value).unwrap();
    }
    output.flush().unwrap();
}

fn extract_sorting_key(value: &str, extract_regex: &Regex) -> Option<i64> {
    let extracted = extract_regex.captures(value)?.get(1)?.as_str();

    extracted.parse().ok()
}
