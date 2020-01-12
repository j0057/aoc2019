use std::collections::{HashMap, HashSet, VecDeque};

use std::io::Write;
use std::iter::successors;

use crate::csiseq;
use crate::intcode;

//
// enum Tile
//

#[derive(Debug, PartialEq)]
enum Tile {
    Unknown,
    Wall,
    Floor,
    Target,
}

impl Tile {
    fn is_unknown(&self) -> bool {
        match *self {
            Tile::Unknown => true,
            _             => false,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Tile::Unknown => "▓▓",
            Tile::Wall    => "██",
            Tile::Floor   => "▒▒",
            Tile::Target  => "XX",
        })
    }
}

//
// struct BBox
//

type Coord = num_complex::Complex<i32>;

#[derive(Debug)]
struct BBox {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32
}

impl BBox {
    fn new() -> Self {
        Self {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0
        }
    }

    fn extend(&mut self, c: Coord) {
        self.min_x = std::cmp::min(self.min_x, c.re);
        self.min_y = std::cmp::min(self.min_y, c.im);
        self.max_x = std::cmp::max(self.max_x, c.re);
        self.max_y = std::cmp::max(self.max_y, c.im);
    }
}

//
// struct Grid
//

static UP: Coord = Coord { re: 0, im: -1 };
static DN: Coord = Coord { re: 0, im: 1 };
static LT: Coord = Coord { re: -1, im: 0 };
static RT: Coord = Coord { re: 1, im: 0 };

lazy_static! {
    static ref DIRECTIONS: HashMap<Coord, i128> = {
        [(UP, 1), (DN, 2), (LT, 3), (RT, 4)]
            .iter()
            .cloned()
            .collect::<HashMap<Coord, i128>>()
    };
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Coord, Tile>,
    bbox: BBox,
    pos: Coord,
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: HashMap::<Coord, Tile>::new(),
            bbox: BBox::new(),
            pos: Coord::new(0, 0),
        }
    }

    fn find_adjacent(&self, coord: Coord, kind: &Tile) -> HashSet<Coord> {
        DIRECTIONS
            .keys()
            .map(|d| coord + d)
            .map(|c| (c, self.grid.get(&c).unwrap_or(&Tile::Unknown)))
            .filter_map(|(c, t)| if t == kind { Some(c) } else { None })
            .collect()
    }

    fn bfs(&self, src: Coord, tgt: Coord) -> Vec<Coord> {
        self.iter_routes(src)
            .filter(|route| route[0] == tgt)
            .nth(0)
            .unwrap()
    }

    fn path_to_closest_unknown(&self, pos: Coord) -> Option<Vec<Coord>> {
        self.iter_routes(pos)
            .filter(|route| self[*route.first().unwrap()] == Tile::Unknown)
            .nth(0)
    }

    fn iter_routes(&self, src: Coord) -> RouteIter {
        RouteIter::new(&self, src)
    }
}

impl std::ops::Index<Coord> for Grid {
    type Output = Tile;

    fn index(&self, idx: Coord) -> &Self::Output {
        self.grid.get(&idx).unwrap_or(&Tile::Unknown)
    }
}

impl std::ops::IndexMut<Coord> for Grid {
    fn index_mut(&mut self, idx: Coord) -> &mut Tile {
        self.bbox.extend(idx);
        self.grid.entry(idx).or_insert(Tile::Unknown)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut c = Coord::new(0, 0);
        for y in self.bbox.min_y..=self.bbox.max_y {
            c.im = y;
            for x in self.bbox.min_x..=self.bbox.max_x {
                c.re = x;
                if c == self.pos {
                    write!(f, "@@")?;
                }
                else {
                    write!(f, "{}", self.grid.get(&c).unwrap_or(&Tile::Unknown).to_string())?;
                }
            }
            if y == self.bbox.min_y {
                write!(f, "    X: {} {}, Y: {} {}", self.bbox.min_x, self.bbox.max_x, self.bbox.min_y, self.bbox.max_y)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

//
// struct RouteIter
//

#[derive(Debug)]
struct RouteIter<'a> {
    grid: &'a Grid,
    queue: VecDeque<Coord>,
    parents: HashMap<Coord, Option<Coord>>,
}

impl<'a> RouteIter<'a> {
    fn new(grid: &'a Grid, pos: Coord) -> Self {
        let mut result = Self {
            grid,
            queue: VecDeque::new(),
            parents: HashMap::new(),
        };
        result.queue.push_back(pos);
        result.parents.insert(pos, None);
        result
    }
}

impl Iterator for RouteIter<'_> {
    type Item = Vec<Coord>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.queue.pop_front() {
            if ! self.grid[v].is_unknown() {
                for k in &[Tile::Unknown, Tile::Floor, Tile::Target] {
                    for w in self.grid.find_adjacent(v, k) {
                        if self.parents.contains_key(&w) {
                            continue;
                        }
                        self.parents.insert(w, Some(v));
                        self.queue.push_back(w);
                    }
                }
            }
            let path = successors(Some(v), |x| *self.parents.get(x)?).collect();
            return Some(path);
        }
        None
    }
}

//
// struct RepairDroid
//

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
struct RepairDroid {
    vm: intcode::VM,
    state: intcode::Status,
    grid: Grid,
    plan: Vec<Coord>,
    step: Coord,
    target: Option<Coord>,
}

impl RepairDroid {
    fn new(vm: &intcode::VM) -> Self {
        let mut result = Self {
            vm: vm.clone(),
            state: intcode::Status::Suspended,
            grid: Grid::new(),
            plan: Vec::<Coord>::new(),
            step: Coord::new(0, 0),
            target: None,
        };
        result.grid[Coord::new(0, 0)] = Tile::Floor;
        result
    }
}

impl Iterator for RepairDroid {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.plan.is_empty() {
            self.plan = match self.grid.path_to_closest_unknown(self.grid.pos) {
                Some(x) => { x }
                None    => { return None }
            };
            self.plan.pop();
        }

        let mut input = Vec::<i128>::new();
        let mut output = Vec::<i128>::new();
        loop {
            match self.vm.step(&mut input, &mut output) {

                intcode::Status::Halted => { break; }

                intcode::Status::Blocked => { self.step = self.plan.pop().unwrap() - self.grid.pos;
                                              input.push(*DIRECTIONS.get(&self.step).unwrap()); }

                intcode::Status::Suspended => { let x = self.grid.pos + self.step;
                                                match output.remove(0) {
                                                    0 => { self.grid[x] = Tile::Wall }
                                                    1 => { self.grid[x] = Tile::Floor  ; self.grid.pos += self.step }
                                                    2 => { self.grid[x] = Tile::Target ; self.grid.pos += self.step ; self.target = Some(self.grid.pos) }
                                                    x => { panic!("HAVE OUTPUT {}", x) }
                                                };
                                                return Some(self.grid.to_string()); }
            }
        }

        None
    }
}

//
// solution
//

pub fn day15a(vm: &intcode::VM) -> i32 {
    let mut droid = RepairDroid::new(vm);
    for _ in &mut droid { }
    droid.grid.bfs(Coord::new(0, 0), droid.target.unwrap()).len() as i32 - 1
}

pub fn day15b(vm: &intcode::VM) -> i32 {
    let mut droid = RepairDroid::new(vm);
    for _ in &mut droid { }
    droid.grid
        .iter_routes(droid.target.unwrap())
        .max_by_key(|route| route.len())
        .unwrap()
        .len() as i32 - 1
}

pub fn day15_main(vm: &intcode::VM) -> Result<(), Error> {
    let mut droid = RepairDroid::new(vm);
    let mut stdout = std::io::stdout();
    stdout.write_all(&csiseq::HIDE_CURSOR)?;
    stdout.write_all(&csiseq::CLEAR_SCREEN)?;
    for s in &mut droid {
        stdout.write_all(&csiseq::move_cursor(1, 1))?;
        stdout.write_all(&s.bytes().collect::<Vec<_>>())?;
        stdout.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    match droid.target {
        Some(x) => stdout.write_all(&(droid.grid.bfs(Coord::new(0, 0), x).len()-1).to_string().bytes().collect::<Vec<_>>())?,
        None    => stdout.write_all(b"Oxygen system not found :-(")?,
    }
    stdout.write_all(&csiseq::SHOW_CURSOR)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::util;

    #[test]
    fn test_15_bbox_1() {
        let mut bbox = BBox { min_x: 0, min_y: 0, max_x: 0, max_y: 0 };
        bbox.extend(Coord::new(13, 0));
        assert_eq!(bbox.max_x, 13);
    }

    #[test]
    fn test_15_grid_1() {
        let mut grid = Grid::new();
        grid[Coord::new(0, 0)] = Tile::Floor;
        assert_eq!(grid[Coord::new(0, 0)], Tile::Floor);
    }

    #[test]
    fn test_15_grid_2() {
        let mut grid = Grid::new();
        grid[Coord::new(0, 0)] = Tile::Floor;
        grid[Coord::new(0, 1)] = Tile::Floor;
        let unknowns = grid.find_adjacent(Coord::new(0, 1), &Tile::Unknown);
        assert_eq!(unknowns, [(-1, 1), (0, 2), (1, 1)]
                               .iter()
                               .map(|&(x, y)| Coord::new(x, y))
                               .collect::<HashSet<Coord>>());
    }

    #[test]
    fn test_15_grid_4() {
        let mut grid = Grid::new();
        grid[Coord::new( 0, 0)] = Tile::Floor;
        grid[Coord::new( 0, 1)] = Tile::Floor;
        grid[Coord::new(-1, 2)] = Tile::Floor;
        grid[Coord::new( 0, 2)] = Tile::Floor;
        grid[Coord::new( 1, 2)] = Tile::Floor;
        let path = grid.bfs(Coord::new(0, 0), Coord::new(2, 2));
        assert_eq!(&path, &[(2, 2), (1, 2), (0, 2), (0, 1), (0, 0)]
                               .iter()
                               .map(|&(x, y)| Coord::new(x, y))
                               .collect::<Vec<_>>());
    }

    #[test]
    fn test_15_grid_5() {
        let mut grid = Grid::new();
        grid[Coord::new(0, 0)] = Tile::Wall; grid[Coord::new(1, 0)] = Tile::Wall;  grid[Coord::new(2, 0)] = Tile::Wall;
        grid[Coord::new(0, 1)] = Tile::Wall; grid[Coord::new(1, 1)] = Tile::Floor; grid[Coord::new(2, 1)] = Tile::Unknown;
        grid[Coord::new(0, 2)] = Tile::Wall; grid[Coord::new(1, 2)] = Tile::Floor; grid[Coord::new(2, 2)] = Tile::Wall;
        grid[Coord::new(0, 3)] = Tile::Wall; grid[Coord::new(1, 3)] = Tile::Floor; grid[Coord::new(2, 3)] = Tile::Wall;
        grid[Coord::new(0, 4)] = Tile::Wall; grid[Coord::new(1, 4)] = Tile::Floor; grid[Coord::new(2, 4)] = Tile::Wall;
        grid[Coord::new(0, 5)] = Tile::Wall; grid[Coord::new(1, 5)] = Tile::Floor; grid[Coord::new(2, 5)] = Tile::Wall;
        grid[Coord::new(0, 6)] = Tile::Wall; grid[Coord::new(1, 6)] = Tile::Floor; grid[Coord::new(2, 6)] = Tile::Unknown;
        grid[Coord::new(0, 7)] = Tile::Wall; grid[Coord::new(1, 7)] = Tile::Wall;  grid[Coord::new(2, 7)] = Tile::Wall;

        let path = grid.path_to_closest_unknown(Coord::new(1, 3)).unwrap();

        assert_eq!(&path, &[(2, 1), (1, 1), (1, 2), (1, 3)]
                               .iter()
                               .map(|&(x, y)| Coord::new(x, y))
                               .collect::<Vec<_>>());
    }

    #[test]
    fn test_15() -> Result<(), Box<dyn std::error::Error>> {
        let input = util::get_parsed_line::<intcode::VM>("input/day15.txt")?;
        assert_eq!(day15a(&input), 330);
        assert_eq!(day15b(&input), 352);
        Ok(())
    }
}
