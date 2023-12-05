use std::{fs::File, io::BufRead, io::BufReader, str::FromStr};

use anyhow::{bail, Context, Error};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Mapping {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

impl Mapping {
    fn includes(&self, input: i64) -> bool {
        input >= self.source_start && input < self.source_start + self.len
    }

    fn map(&self, input: i64) -> i64 {
        if !self.includes(input) {
            return input;
        }

        let offset = self.dest_start - self.source_start;
        input + offset
    }
}

impl FromStr for Mapping {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(' ');

        let dest_start = it.next().context("failed to read dest start")?.parse()?;
        let source_start = it.next().context("failed to read source start")?.parse()?;
        let len = it.next().context("failed to read len")?.parse()?;
        if it.next().is_some() {
            bail!("unexpected token");
        }

        Ok(Self {
            dest_start,
            source_start,
            len,
        })
    }
}

#[derive(Debug)]
struct Mapper {
    mappings: Vec<Mapping>,
    name: String,
}

impl Mapper {
    fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            mappings: Vec::new(),
            name: name.as_ref().to_owned(),
        }
    }

    fn resolve(&self, input: i64) -> i64 {
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.includes(input)) {
            mapping.map(input)
        } else {
            input
        }
    }
}

struct FullMapper {
    mappers: Vec<Mapper>,
}

impl FullMapper {
    fn resolve(&self, input: i64) -> i64 {
        let mut current = input;

        for mapper in &self.mappers {
            current = mapper.resolve(current);
        }

        current
    }
}

#[derive(Debug)]
enum ParserState {
    Init,
    Idle,
    InMapping,
}

#[derive(Debug)]
struct Parser {
    state: ParserState,
    seeds: Vec<i64>,
    mappers: Vec<Mapper>,
}

impl Parser {
    fn new() -> Self {
        Self {
            state: ParserState::Init,
            seeds: Vec::new(),
            mappers: Vec::new(),
        }
    }

    fn finalize(self) -> (Vec<i64>, FullMapper) {
        (
            self.seeds,
            FullMapper {
                mappers: self.mappers,
            },
        )
    }

    fn parse_line(&mut self, line: &str) -> Result<(), Error> {
        match self.state {
            ParserState::Init => {
                if let Some(rest) = line.strip_prefix("seeds: ") {
                    for seed in rest.split_ascii_whitespace() {
                        let seed: i64 = seed.parse()?;
                        self.seeds.push(seed);
                    }

                    self.state = ParserState::Idle;
                } else {
                    bail!("invalid input: {line}");
                }
            }
            ParserState::Idle => {
                if line.is_empty() {
                    return Ok(());
                }

                static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([-\w]+) map:").unwrap());
                if let Some(captures) = RE.captures(line) {
                    let name = captures.get(1).unwrap().as_str();
                    self.mappers.push(Mapper::new(name));
                    self.state = ParserState::InMapping;
                } else {
                    bail!("invalid input: {line}")
                }
            }
            ParserState::InMapping => {
                if line.is_empty() {
                    self.state = ParserState::Idle;
                    return Ok(());
                }

                let mapping: Mapping = line.parse()?;

                self.mappers
                    .last_mut()
                    .context("invalid state")?
                    .mappings
                    .push(mapping);
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut parser = Parser::new();

    let file = std::env::args().nth(1).context("missing argument")?;
    let file = File::open(file)?;

    for line in BufReader::new(file).lines() {
        let line = line?;

        parser.parse_line(&line)?;
    }

    let (seeds, full_mapper) = parser.finalize();
    let min = seeds.iter().map(|seed| full_mapper.resolve(*seed)).min();
    println!("Part 1, Minimum location: {min:?}");

    let min = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .flat_map(|(start, len)| (*start..*start + *len).map(|i| full_mapper.resolve(i)))
        .min();
    println!("Part 2, Minimum location: {min:?}");

    Ok(())
}

#[test]
fn test_resolve_multiple() {
    let mapping1 = Mapping {
        source_start: 10,
        dest_start: 20,
        len: 2,
    };
    let mapping2 = Mapping {
        source_start: 90,
        dest_start: 80,
        len: 10,
    };

    let mapper = Mapper {
        mappings: vec![mapping1, mapping2],
        name: String::from("test"),
    };

    assert_eq!(mapper.resolve(10), 20);
    assert_eq!(mapper.resolve(11), 21);
    assert_eq!(mapper.resolve(12), 12);
    assert_eq!(mapper.resolve(50), 50);
    assert_eq!(mapper.resolve(89), 89);
    assert_eq!(mapper.resolve(90), 80);
    assert_eq!(mapper.resolve(99), 89);
    assert_eq!(mapper.resolve(100), 100);
}

#[test]
fn test_parser() {
    let mut parser = Parser::new();

    parser.parse_line("seeds: 10 20 30").unwrap();
    parser.parse_line("").unwrap();
    parser.parse_line("seed-to-soil map:").unwrap();
    parser.parse_line("10 10 10").unwrap();
    parser.parse_line("20 20 20").unwrap();
    parser.parse_line("").unwrap();
    parser.parse_line("soil-to-humidity map:").unwrap();
    parser.parse_line("10 10 10").unwrap();
    parser.parse_line("20 20 20").unwrap();
}
