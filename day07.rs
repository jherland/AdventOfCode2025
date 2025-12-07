use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct BeamTrace {
    row: usize,
    beams: HashMap<usize, usize>,
}

fn add_beams(beams: &mut HashMap<usize, usize>, beam: usize, count: usize) {
    let new_count = count + match beams.get(&beam) {
        None => 0,
        Some(c) => *c,
    };
    beams.insert(beam, new_count);
}

#[derive(Debug)]
struct Manifold {
    map: Vec<Vec<char>>,
}

impl Manifold {
    fn parse(lines: impl Iterator<Item = String>) -> Self
    {
        Self { map: lines.map(|line| line.chars().collect()).collect() }
    }

    fn rows(&self) -> usize {
        self.map.len()
    }

    fn start_trace(&self) -> BeamTrace {
        BeamTrace {
            row: 0,
            beams: HashMap::from([(self.map[0].iter().position(|c| *c == 'S').unwrap(), 1)]),
        }
    }

    fn advance_trace(&self, trace: &BeamTrace) -> (BeamTrace, usize) {
        let row = trace.row + 1;
        assert!(row < self.rows());
        let mut beams = HashMap::new();
        let mut num_splits = 0;
        for (beam, count) in &trace.beams {
            if self.map[row][*beam] == '^' { // split!
                num_splits += 1;
                add_beams(&mut beams, *beam - 1, *count);
                add_beams(&mut beams, *beam + 1, *count);
            }
            else { // pass on
                add_beams(&mut beams, *beam, *count);
            }
        }
        ( BeamTrace { row, beams }, num_splits)
    }
}

fn main() {
    let manifold = Manifold::parse(io::stdin().lines().map(Result::unwrap));
    let mut trace = manifold.start_trace();
    let mut split_count = 0;
    while trace.row < manifold.rows() - 1 {
        let (next_trace, num_splits) = manifold.advance_trace(&trace);
        split_count += num_splits;
        trace = next_trace;
    }
    println!("Part 1: {}", split_count); // 1543
    println!("Part 2: {}", trace.beams.into_values().sum::<usize>()); // 3223365367809
}
