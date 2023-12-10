use std::collections::HashSet;

use anyhow::{anyhow, bail, Context, Error};
use common_rust::read_lines;

#[derive(Debug, Clone, Copy)]
enum Pipe {
    Horizontal,
    Vertical,
    BendSouthWest,
    BendSouthEast,
    BendNorthWest,
    BendNorthEast,
    NoPipe,
}

impl Pipe {
    fn next_heading(self, heading: Heading) -> Result<Heading, Error> {
        match (heading, self) {
            (Heading::North, Pipe::Vertical) => Ok(Heading::North),
            (Heading::North, Pipe::BendSouthWest) => Ok(Heading::West),
            (Heading::North, Pipe::BendSouthEast) => Ok(Heading::East),
            (Heading::South, Pipe::Vertical) => Ok(Heading::South),
            (Heading::South, Pipe::BendNorthWest) => Ok(Heading::West),
            (Heading::South, Pipe::BendNorthEast) => Ok(Heading::East),
            (Heading::East, Pipe::Horizontal) => Ok(Heading::East),
            (Heading::East, Pipe::BendSouthWest) => Ok(Heading::South),
            (Heading::East, Pipe::BendNorthWest) => Ok(Heading::North),
            (Heading::West, Pipe::Horizontal) => Ok(Heading::West),
            (Heading::West, Pipe::BendSouthEast) => Ok(Heading::South),
            (Heading::West, Pipe::BendNorthEast) => Ok(Heading::North),
            (_, _) => Err(anyhow!("no path")),
        }
    }

    fn headings(self) -> [Heading; 2] {
        use Heading::*;

        match self {
            Pipe::Horizontal => [East, West],
            Pipe::Vertical => [South, North],
            Pipe::BendSouthWest => [West, South],
            Pipe::BendSouthEast => [East, South],
            Pipe::BendNorthWest => [West, North],
            Pipe::BendNorthEast => [North, East],
            Pipe::NoPipe => panic!("invalid"),
        }
    }
}

impl TryFrom<&[Heading; 2]> for Pipe {
    type Error = Error;

    fn try_from(value: &[Heading; 2]) -> Result<Self, Self::Error> {
        use Pipe::*;

        Ok(match (value[0], value[1]) {
            (Heading::North, Heading::East) => BendNorthEast,
            (Heading::North, Heading::South) => Vertical,
            (Heading::North, Heading::West) => BendNorthWest,
            (Heading::East, Heading::North) => BendNorthEast,
            (Heading::East, Heading::South) => BendSouthEast,
            (Heading::East, Heading::West) => Horizontal,
            (Heading::South, Heading::North) => Vertical,
            (Heading::South, Heading::East) => BendSouthEast,
            (Heading::South, Heading::West) => BendSouthWest,
            (Heading::West, Heading::North) => BendNorthWest,
            (Heading::West, Heading::East) => Horizontal,
            (Heading::West, Heading::South) => BendSouthWest,
            _ => bail!("invalid"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn to_offsets(self) -> (i64, i64) {
        match self {
            Heading::North => (-1, 0),
            Heading::East => (0, 1),
            Heading::South => (1, 0),
            Heading::West => (0, -1),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::Horizontal),
            '|' => Ok(Self::Vertical),
            'L' => Ok(Self::BendNorthEast),
            'J' => Ok(Self::BendNorthWest),
            '7' => Ok(Self::BendSouthWest),
            'F' => Ok(Self::BendSouthEast),
            'S' => Ok(Self::NoPipe),
            '.' => Ok(Self::NoPipe),
            _ => Err(anyhow::anyhow!("invalid pipe")),
        }
    }
}

struct SolverBuilder {
    pipes: Vec<Vec<Pipe>>,
    start: Option<(usize, usize)>,
}

impl SolverBuilder {
    fn new() -> Self {
        SolverBuilder {
            pipes: Vec::new(),
            start: None,
        }
    }

    fn process_line(&mut self, line: &str) -> Result<(), Error> {
        let mut pipes = Vec::new();

        for (column, ch) in line.chars().enumerate() {
            let pipe: Pipe = ch.try_into()?;

            if ch == 'S' {
                match self.start {
                    Some(start) => bail!("There is already a start {start:?}"),
                    None => self.start = Some((self.pipes.len(), column)),
                }
            }

            pipes.push(pipe);
        }

        self.pipes.push(pipes);

        Ok(())
    }

    fn build(mut self) -> Result<Solver, Error> {
        use Heading::*;

        let start = self.start.context("no start")?;

        let mut connected_dirs = [North, North];
        let mut index = 0;

        for heading in [North, East, South, West] {
            let offset = heading.to_offsets();

            let neighbor_row = start.0 as i64 + offset.0;
            let neighbor_col = start.1 as i64 + offset.1;

            if neighbor_row < 0 || neighbor_row as usize >= self.pipes.len() {
                continue;
            }
            if neighbor_col < 0 || neighbor_col as usize >= self.pipes[neighbor_row as usize].len()
            {
                continue;
            }

            let neighbor = self.pipes[neighbor_row as usize][neighbor_col as usize];

            if neighbor.next_heading(heading).is_ok() {
                if index > 1 {
                    bail!("invalid start");
                }
                connected_dirs[index] = heading;
                index += 1;
            }
        }

        let start_pipe = (&connected_dirs).try_into()?;
        self.pipes[start.0][start.1] = start_pipe;

        Ok(Solver {
            pipes: self.pipes,
            start,
        })
    }
}

#[derive(Debug)]
struct Solver {
    pipes: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl Solver {
    fn solve(&self) -> Result<i32, Error> {
        let start_pipe = self.pipes[self.start.0][self.start.1];
        let mut visited = HashSet::new();

        let mut heading = start_pipe.headings()[0];

        let mut row = self.start.0;
        let mut col = self.start.1;
        let mut steps = 1;

        loop {
            let offset = heading.to_offsets();
            visited.insert((row, col));

            row = (row as i64 + offset.0) as usize;
            col = (col as i64 + offset.1) as usize;
            if col == self.start.1 && row == self.start.0 {
                break;
            }

            let pipe = self.pipes[row][col];
            heading = pipe.next_heading(heading)?;

            steps += 1;
        }

        let mut fields = 0;

        for row in 0..self.pipes.len() {
            let mut inside = false;
            for col in 0..self.pipes[row].len() {
                if visited.contains(&(row, col)) {
                    let pipe = self.pipes[row][col];

                    match &pipe {
                        Pipe::BendNorthWest | Pipe::Vertical | Pipe::BendNorthEast => {
                            inside = !inside;
                        }
                        _ => {}
                    }
                } else if inside {
                    fields += 1;
                }
            }
        }

        println!("{fields}");

        Ok(steps / 2)
    }
}

fn main() -> Result<(), Error> {
    let a = std::env::args().nth(1).context("Input file missing")?;

    let mut builder = SolverBuilder::new();

    for line in read_lines(a)? {
        let line = line?;
        builder.process_line(&line)?;
    }

    let solver = builder.build()?;

    let solution = solver.solve()?;

    println!("{solution}");

    Ok(())
}
