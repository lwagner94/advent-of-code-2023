use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    num::ParseIntError,
    path::Path,
    str::FromStr,
    vec,
};

use anyhow::{Error, Result};

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(Debug)]
struct Calculator {
    vals: Vec<Vec<i64>>,
}

impl FromStr for Calculator {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let first_line: Result<Vec<i64>, ParseIntError> =
            s.split_whitespace().map(|s| s.parse::<i64>()).collect();

        Ok(Self {
            vals: vec![first_line?],
        })
    }
}

impl Calculator {
    fn calculate(&mut self, is_part_2: bool) -> i64 {
        if is_part_2 {
            self.vals[0].reverse();
        }

        let mut current_layer = 0;
        loop {
            let mut next = Vec::new();
            let mut all_zero = true;

            for i in 0..self.vals[current_layer].len() - 1 {
                let diff = self.vals[current_layer][i + 1] - self.vals[current_layer][i];

                if diff != 0i64 {
                    all_zero = false;
                }
                next.push(diff);
            }

            current_layer += 1;
            self.vals.push(next);
            if all_zero {
                break;
            }
        }

        self.vals[current_layer].push(0);
        current_layer -= 1;

        loop {
            let new_last = self.vals[current_layer].last().unwrap()
                + self.vals[current_layer + 1].last().unwrap();
            self.vals[current_layer].push(new_last);
            if current_layer == 0 {
                break;
            }

            current_layer -= 1;
        }

        *self.vals[0].last().unwrap()
    }
}

fn main() -> Result<(), Error> {
    let mut sum = 0;

    for line in read_lines("input")? {
        let line = line?;

        let mut c: Calculator = line.parse()?;
        sum += c.calculate(true);
    }

    println!("{sum}");

    Ok(())
}
