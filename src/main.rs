use std::{io::{stdin, BufRead, stdout, Write, BufWriter, stderr, self}, process::ExitCode};

use pico_args::Arguments;
use regex::Regex;

mod strings;
mod errors;

use errors::{MainError, argerr_transform, print_error};

struct Sortable {
    value: String,
    key: i64,
}
fn main() -> ExitCode {
    let args = Arguments::from_env();

    match app(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => match e {
            MainError::Output(ref io_err) => match io_err.kind() {
                io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
                _ => {
                    print_error(e)
                }
            }
            _ => {
                print_error(e)
            }
        }
    }
}

fn app(mut args: Arguments) -> Result<(), MainError> {
    if args.contains("--help") || args.contains("-h") {
        stderr().write_all(strings::HELP.as_bytes())?;
        return Ok(()).into()
    }

    let reverse = args.contains("--reverse") || args.contains("-r");
    
    let subgroup: usize = args.opt_value_from_str("--subgroup").unwrap()
        .or(args.opt_value_from_str("-s").unwrap())
        .unwrap_or(1);

    let sort_by_regex: String = args.free_from_str().map_err(argerr_transform("PATTERN"))?;

    let input = stdin().lock();

    let mut extracted = extract(
        input.lines().map(Result::unwrap),
        regex_extraction(&sort_by_regex, subgroup),
    );

    sort(&mut extracted, reverse);

    let mut out = BufWriter::new(stdout().lock());

    output(extracted.iter().map(|v| v.value.as_str()), &mut out)?;

    Ok(()).into()
}

#[must_use]
fn extract(lines: impl Iterator<Item = String>, mut extraction: impl FnMut(&str) -> Option<i64>) -> Vec<Sortable> {
    lines
        .filter_map(|value| {
            let Some(key) = extraction(&value) else {
                return None
            };
            Some(Sortable {
                value,
                key,
            })
        })
        .collect()
}

fn sort<'a>(input: &mut [Sortable], reverse: bool) {
    input.sort_by(|a, b| {
        let ordering = a.key.cmp(&b.key);

        if reverse {
            ordering.reverse()
        } else {
            ordering
        }
    });
}

fn output<'a, W: Write>(lines: impl Iterator<Item = &'a str>, output: &mut BufWriter<W>) -> Result<(), io::Error> {
    for i in lines {
        let r = writeln!(output, "{}", i);

        // Make sure we always flush the output. (Not sure if it's necessary.)
        if let Err(e) = r {
            match e.kind() {
                io::ErrorKind::BrokenPipe => break,
                _ => Err(e)?,
            }
        }
    }

    output.flush()?;
    Ok(())
}

fn regex_extraction(sort_by_regex: &str, subgroup: usize) -> impl FnMut(&str) -> Option<i64> {
    let sort_by = Regex::new(sort_by_regex).unwrap();

    move |value| {
        let extracted = sort_by.captures(value)?.get(subgroup)?.as_str();

        extracted.parse().ok()
    }
}