use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn nbors(&self) -> impl Iterator<Item = Self> + use<'_> {
        let y_start = if self.y == 0 { 0 } else { self.y - 1 };
        let x_start = if self.x == 0 { 0 } else { self.x - 1 };
        (y_start..=self.y + 1)
            .map(move |y| (x_start..=self.x + 1)
                .map(move |x| Self { y, x })
                .filter(move |pos| pos != self)
            )
            .flatten()
    }
}

#[derive(Debug)]
struct World {
    map: Vec<Vec<bool>>,
    size: Pos,
}

impl World {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut width = None;
        let mut map = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for byte in line.as_bytes().into_iter() {
                row.push(*byte == b'@');
            }
            match width {
                None => width = Some(row.len()),
                Some(n) => assert!(n == row.len()),
            }
            map.push(row);
        }
        let size = Pos { y: map.len(), x: width.unwrap() };
        Self { map, size }
    }

    fn contains(&self, p: Pos) -> bool {
        p.y < self.size.y && p.x < self.size.x
    }

    fn is_occupied(&self, p: Pos) -> bool {
        if ! self.contains(p) { return false; }
        self.map[p.y][p.x]
    }

    fn adjacents_occupied(&self, p: Pos) -> usize {
        p.nbors().filter(|nbor| self.is_occupied(*nbor)).count()
    }

    fn iter_occupied(&self) -> impl Iterator<Item = Pos> + use<'_> {
        (0..self.size.y)
            .map(move |y| (0..self.size.x)
                .map(move |x| Pos { y, x })
            )
            .flatten()
            .filter(move |pos| self.is_occupied(*pos))
    }

    fn num_occupied(&self) -> usize {
        self.iter_occupied().count()
    }

    fn accessible(&self) -> impl Iterator<Item = Pos> + use<'_> {
        self
            .iter_occupied()
            .filter(|pos| self.adjacents_occupied(*pos) < 4)
    }

    fn remove(&self, positions: impl Iterator<Item = Pos>) -> Self {
        let mut new_map: Vec<Vec<bool>> = self.map.iter().map(|row| row.clone()).collect();
        for pos in positions {
            new_map[pos.y][pos.x] = false;
        }
        Self {map: new_map, size: self.size}
    }
}

fn main() {
    let mut world = World::parse(io::stdin().lines().map(Result::unwrap));
    println!("Part 1: {}", world.accessible().count()); // 1547

    let initial_rolls = world.num_occupied();
    let mut prev_rolls = initial_rolls;
    loop {
        world = world.remove(world.accessible());
        let rolls = world.num_occupied();
        if rolls == prev_rolls { // nothing was removed
            break;
        }
        else {
            prev_rolls = rolls;
        }
    }
    println!("Part 2: {}", initial_rolls - world.num_occupied()); // 8948
}
