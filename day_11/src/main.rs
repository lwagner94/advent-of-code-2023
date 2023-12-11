use std::fmt::Display;
use std::{char, fmt::Write, path::Path};

use anyhow::{bail, Error};

#[derive(Clone, Copy)]
enum Element {
    Galaxy,
    Empty,
}

impl TryFrom<char> for Element {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::Galaxy,
            '.' => Self::Empty,
            _ => bail!("invalid character"),
        })
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Galaxy => f.write_char('#'),
            Element::Empty => f.write_char('.'),
        }
    }
}

struct Universe {
    space: Vec<Vec<Element>>,
    expansion_factor: usize,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.space {
            for element in line {
                f.write_fmt(format_args!("{element}"))?;
            }

            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Universe {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut space = Vec::new();

        for line in common_rust::read_lines(path)? {
            let mut row = Vec::new();

            for ch in line?.chars() {
                let element = ch.try_into()?;
                row.push(element);
            }
            space.push(row);
        }

        Ok(Self {
            space,
            expansion_factor: 2,
        })
    }

    fn distances(&self) -> i64 {
        let mut rows: Vec<bool> = vec![true; self.space.len()];
        let mut cols: Vec<bool> = vec![true; self.space[0].len()];
        let mut galaxies = Vec::new();

        for (y, line) in self.space.iter().enumerate() {
            for (x, element) in line.iter().enumerate() {
                if matches!(element, Element::Galaxy) {
                    galaxies.push((y, x));
                    rows[y] = false;
                    cols[x] = false;
                }
            }
        }

        let mut sum = 0;

        for i in 0..galaxies.len() - 1 {
            for j in i + 1..galaxies.len() {
                let first = galaxies[i];
                let second = galaxies[j];

                let min_y = first.0.min(second.0);
                let max_y = first.0.max(second.0);
                let min_x = first.1.min(second.1);
                let max_x = first.1.max(second.1);

                let empty_cols =
                    (min_y + 1..max_y)
                        .fold(0, |acc, index| if rows[index] { acc + 1 } else { acc });

                let empty_rows =
                    (min_x + 1..max_x)
                        .fold(0, |acc, index| if cols[index] { acc + 1 } else { acc });

                let diff_x = max_x as i64 - min_x as i64;
                let diff_y = max_y as i64 - min_y as i64;

                let expand_y = empty_rows * (self.expansion_factor - 1);
                let expand_x = empty_cols * (self.expansion_factor - 1);
                let distance = diff_x + diff_y + expand_x as i64 + expand_y as i64;

                sum += distance;
            }
        }

        sum
    }
}

fn main() -> Result<(), Error> {
    let file = std::env::args().nth(1).expect("no input file provided");

    let mut universe = Universe::from_file(file)?;

    println!("Part 1: {}", universe.distances());
    universe.expansion_factor = 1000000;
    println!("Part 2: {}", universe.distances());

    Ok(())
}
