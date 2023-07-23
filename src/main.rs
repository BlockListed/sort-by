use std::{io::{stdin, BufRead, stdout, Write}, env::args};

use regex::Regex;

fn main() {
    let input = stdin().lock();

    let sort_by_regex = args().nth(1).unwrap();

    let sort_by = Regex::new(&sort_by_regex).unwrap();

    let mut lines: Vec<_> = input.lines().map(|l| l.unwrap()).filter(|v| !v.is_empty()).collect();

    eprintln!("Acquired lines");

    lines.sort_by(|a, b| {
        let a = extract_sorting_key(a, &sort_by);
        let b = extract_sorting_key(b, &sort_by);

        // Switched around, because we want descending sorts
        std::cmp::Ord::cmp(&b, &a)
    });

    eprintln!("Sorted");

    let mut output = stdout().lock();
    for i in lines {
        writeln!(&mut output, "{}", i).unwrap();
    }
}

fn extract_sorting_key(value: &str, extract_regex: &Regex) -> i64 {
    let extracted = extract_regex.captures(value).unwrap().get(1).unwrap().as_str();

    extracted.parse().unwrap()
}
