use std::io::{stdin, BufRead, stdout, Write, BufWriter, stderr};

use pico_args::Arguments;
use regex::Regex;

mod strings;
mod errors;

use errors::{ArgumentIntoMain, IntoMainResult, MyResult};

struct Sortable {
    value: String,
    key: i64,
}

// "Or", which doesn't short-circuit.
fn or(a: bool, b: bool) -> bool {
    a || b
}

fn main() -> MyResult<(), errors::MainError> {
    let mut args = Arguments::from_env();

    if or(args.contains("--help"), args.contains("-h")) {
        my_try!(stderr().write_all(strings::HELP.as_bytes()));
        return Ok(()).into()
    }

    let reverse = or(args.contains("--reverse"), args.contains("-r"));
    
    let subgroup: usize = args.opt_value_from_str("--subgroup").unwrap()
        .or(args.opt_value_from_str("-s").unwrap())
        .unwrap_or(1);

    let sort_by_regex: String = my_try!(args.free_from_str().into_main("PATTERN"));

    let input = stdin().lock();

    let mut extracted = extract(
        input.lines().map(Result::unwrap),
        regex_extraction(&sort_by_regex, subgroup),
    );

    sort(&mut extracted, reverse);

    let mut out = BufWriter::new(stdout().lock());

    my_try!(output(extracted.iter().map(|v| v.value.as_str()), &mut out).into_main());

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

fn output<'a, W: Write>(lines: impl Iterator<Item = &'a str>, output: &mut BufWriter<W>) -> Result<(), errors::OutputError> {
    for i in lines {
        let r: Result<(), errors::OutputError> = writeln!(output, "{}", i).map_err(|e| e.into());

        // Make sure we always flush the output. (Not sure if it's necessary.)
        if let Err(e) = r {
            match e {
                errors::OutputError::Closed => break,
                errors::OutputError::Other(_) => Err(e)?,
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