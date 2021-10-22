use std::io::{self, BufRead};

use regex::Regex;

struct FnCall {
    name: String,
    params: String,
}

fn main() {
    let stdin = io::stdin();

    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
    {
        // split line on ", " to get expected and actual
        let mut iter = line.split(", ");

        println!("Expected:");
        let expected = parse_fns(iter.next().unwrap());
        print_fns(expected.iter());

        println!("Actual:");
        let actual = parse_fns(iter.next().unwrap());
        print_fns(actual.iter());

        println!("");
    }
}

fn print_fns<'a>(fns: impl Iterator<Item = &'a FnCall>) {
    for f in fns {
        println!("  {}({})", f.name, f.params);
    }
}

fn parse_fns(inp: &str) -> Vec<FnCall> {
    let re = Regex::new(r#"\[([^\(]+)\(([^\)]+)\)\]"#).unwrap();
    let mut fns = vec![];
    for cap in re.captures_iter(inp) {
        fns.push(FnCall {
            name: cap[1].to_string(),
            params: cap[2].to_string(),
        });
    }

    fns
}
