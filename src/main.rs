use std::io::{stdin, BufRead, stdout, Write, BufWriter, ErrorKind};

use pico_args::Arguments;
use regex::Regex;

struct Sortable {
    value: String,
    key: i64,
}

fn main() {
    let mut args = Arguments::from_env();

    let reverse = args.contains("-r") || args.contains("--reverse");

    let sort_by_regex: String = args.free_from_str().unwrap();

    sort(regex_extraction(&sort_by_regex), reverse);
}

fn sort(mut extraction: impl FnMut(&str) -> Option<i64>, reverse: bool) {
    let input = stdin().lock();

    let mut lines: Vec<_> = input
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|value| {
            let Some(key) = extraction(&value) else {
                return None
            };
            Some(Sortable {
                value,
                key,
            })
        })
        .collect();

    lines.sort_by(|a, b| {
        // Switched around, because we want descending sorts
        let ordering = a.key.cmp(&b.key);

        if reverse {
            ordering.reverse()
        } else {
            ordering
        }
    });

    let mut output = BufWriter::new(stdout().lock());
    for i in lines {
        match writeln!(&mut output, "{}", i.value) {
            Ok(_) => (),
            Err(e) => {
                match e.kind() {
                    // This happens when using `head` for example.
                    ErrorKind::BrokenPipe => return,
                    e => panic!("{}", e),
                }
            }
        };
    }
    output.flush().unwrap();
}

fn regex_extraction(sort_by_regex: &str) -> impl FnMut(&str) -> Option<i64> {
    let sort_by = Regex::new(sort_by_regex).unwrap();

    move |value| {
        let extracted = sort_by.captures(value)?.get(1)?.as_str();

        extracted.parse().ok()
    }
}