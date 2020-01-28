use std::collections::{HashMap, BTreeSet, VecDeque};
use std::convert::TryFrom;
use std::io::Write;

//
// struct Coord
//

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord(i32, i32);

impl Coord {
    fn neighbours(&self) -> [Coord; 4] {
        [Coord(self.0-1, self.1),
         Coord(self.0, self.1-1),
         Coord(self.0+1, self.1),
         Coord(self.0, self.1+1)]
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

//
// enum ParseErorr
//

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown character: {0:?}")]
    UnknownCharacter(char),
}

//
// enum Tile
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Entrance,
    Wall,
    Floor,
    Door(u8),
    Key(u8),
}

impl std::convert::TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(chr: char) -> Result<Self, Self::Error> {
        match chr {
            '@'       => Ok(Tile::Entrance),
            '#'       => Ok(Tile::Wall),
            '.'       => Ok(Tile::Floor),
            'A'..='Z' => Ok(Tile::Door(chr as u8 - 64)),
            'a'..='z' => Ok(Tile::Key(chr as u8 - 96)),
             _        => Err(ParseError::UnknownCharacter(chr)),
        }
    }
}

impl std::convert::From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Entrance  => '@',
            Tile::Wall      => '█',
            Tile::Floor     => ' ',
            Tile::Door(n)   => (n + 64) as char,
            Tile::Key(n)    => (n + 96) as char,
        }
    }
}

//
// struct Maze
//

#[derive(Debug, Default)]
pub struct Maze {
    grid: HashMap<Coord, Tile>,
    start: Coord,
    keys: BTreeSet<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CacheEntry(Coord, BTreeSet<u8>);

impl Maze {
    fn new(grid: HashMap<Coord, Tile>) -> Self {
        let start = Self::find_entrance(&grid);
        let keys = Self::find_keys(&grid);
        Self {
            grid,
            start,
            keys
        }
    }

    fn find_entrance(grid: &HashMap<Coord, Tile>) -> Coord {
        *grid.iter().find(|&(_, t)| *t == Tile::Entrance).unwrap().0
    }

    fn find_keys(grid: &HashMap<Coord, Tile>) -> BTreeSet<u8> {
        grid.iter().filter_map(|(_, tile)| match tile {
            Tile::Key(k)    => Some(k),
            _               => None
        }).copied().collect::<BTreeSet<_>>()
    }

    fn search<'a>(&'a self, visited: &'a BTreeSet<u8>, source: Coord) -> BreadthFirstSearch<'a> {
        BreadthFirstSearch::new(&self.grid, visited, source)
    }

    fn hash_add(&self, visited: &BTreeSet<u8>, key_pos: &Coord) -> BTreeSet<u8> {
        let key = match self.grid.get(key_pos).unwrap() {
            Tile::Key(k) => *k,
            wtf          => panic!(format!("not a key at {:?}: {:?}", key_pos, wtf))

        };
        let mut result = visited.clone();
        result.insert(key);
        result
    }

    fn shortest_path(&self, source: Coord, visited: BTreeSet<u8>, cache: &mut HashMap<CacheEntry, u64>) -> u64 {
        if visited == self.keys {
            0
        }
        else {
            let entry = CacheEntry(source.clone(), visited.clone());
            if ! cache.contains_key(&entry) {
                let len = self.search(&visited, source)
                    .map(|(target, dist)| dist + self.shortest_path(target, self.hash_add(&visited, &target), cache))
                    .min()
                    .unwrap();
                cache.insert(entry.clone(), len);
            }
            *cache.get(&entry).unwrap()
        }
    }
}

impl std::str::FromStr for Maze {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid = text
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .map(move |(x, chr)| Ok((Coord(x as i32, y as i32), Tile::try_from(chr)?))))
            .collect::<Result<HashMap<Coord, Tile>, ParseError>>()?;
        Ok(Maze::new(grid))
    }
}

impl AsRef<Maze> for Maze {
    fn as_ref(&self) -> &Self {
        &self
    }
}

//
// struct BreadthFirstSearch
//

struct BreadthFirstSearch<'a> {
    grid: &'a HashMap<Coord, Tile>,
    queue: VecDeque<Coord>,
    distance: HashMap<Coord, u64>,
    visited: &'a BTreeSet<u8>
}

impl<'a> BreadthFirstSearch<'a> {
    fn new(grid: &'a HashMap<Coord, Tile>, visited: &'a BTreeSet<u8>, start: Coord) -> Self {
        Self {
            grid,
            queue: [start].iter().copied().collect(),
            distance: [(start, 0)].iter().copied().collect(),
            visited
        }
    }

    fn enqueue(&mut self, pos: Coord, parent: Coord) {
        self.queue.push_back(pos);
        self.distance.insert(pos, self.distance.get(&parent).unwrap() + 1);
    }
}

impl Iterator for BreadthFirstSearch<'_> {
    type Item = (Coord, u64);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.queue.pop_front() {
            match self.grid.get(&v).unwrap() {
                Tile::Key(k) if ! self.visited.contains(k)      => return Some((v, *self.distance.get(&v).unwrap())),
                _                                               => (),
            }
            for &w in v.neighbours().iter() {
                if self.distance.contains_key(&w) {
                    continue;
                }
                match *self.grid.get(&w).unwrap() {
                    Tile::Entrance                              => self.enqueue(w, v),
                    Tile::Floor                                 => self.enqueue(w, v),
                    Tile::Wall                                  => (),
                    Tile::Door(d) if self.visited.contains(&d)  => self.enqueue(w, v),
                    Tile::Door(_)                               => (),
                    Tile::Key(_)                                => self.enqueue(w, v),
                }
            }
        }
        None
    }
}

//
// solution
//

pub fn day18a(maze: &Maze) -> u64 {
    maze.shortest_path(maze.start, BTreeSet::<u8>::new(), &mut HashMap::<_, _>::new())
}

pub fn day18_main(maze: &Maze) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let max_x = maze.grid.keys().map(|c| c.0).max().unwrap();
    let max_y = maze.grid.keys().map(|c| c.1).max().unwrap();
    for y in 0..=max_y {
        stdout.write_all(&(0..=max_x)
            .map(|x| char::from(*maze.grid.get(&Coord(x, y)).unwrap()))
            .collect::<String>()
            .bytes()
            .collect::<Vec<u8>>())?;
        stdout.write_all(&[10])?;
    }
    Ok(())
}

//
// macros
//

#[cfg(test)]
macro_rules! btreeset {
    ($($val: expr),*) => {{
        let mut set = ::std::collections::BTreeSet::new();
        $( set.insert($val); )*
        set
    }}
}

//
// tests
//

#[cfg(test)]
mod test {
    use super::*;

    static EX1: &str = "#########\n\
                        #b.A.@.a#\n\
                        #########";

    static EX2: &str = "########################\n\
                        #f.D.E.e.C.b.A.@.a.B.c.#\n\
                        ######################.#\n\
                        #d.....................#\n\
                        ########################";

    static EX3: &str = "########################\n\
                        #...............b.C.D.f#\n\
                        #.######################\n\
                        #.....@.a.B.c.d.A.e.F.g#\n\
                        ########################";

    static EX4: &str = "#################\n\
                        #i.G..c...e..H.p#\n\
                        ########.########\n\
                        #j.A..b...f..D.o#\n\
                        ########@########\n\
                        #k.E..a...g..B.n#\n\
                        ########.########\n\
                        #l.F..d...h..C.m#\n\
                        #################";

    static EX5: &str = "########################\n\
                        #@..............ac.GI.b#\n\
                        ###d#e#f################\n\
                        ###A#B#C################\n\
                        ###g#h#i################\n\
                        ########################";

    #[test]
    fn test_18_1() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX1.parse()?;
        let result = day18a(&maze);
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    #[allow(unused_mut)]
    fn test_18_2_bfs_start() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX2.parse::<Maze>()?;
        let result = maze.search(&btreeset![], Coord(15, 1)).count();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_18_2_bfs_a() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX2.parse::<Maze>()?;
        let result = maze.search(&btreeset![1], Coord(17, 1)).count();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_18_2_bfs_b() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX2.parse::<Maze>()?;
        let result = maze.search(&btreeset![1, 2], Coord(11, 1)).count();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_18_2_bfs_c() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX2.parse::<Maze>()?;
        let result = maze.search(&btreeset![1, 2, 3], Coord(21, 1)).count();
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn test_18_2() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX2.parse()?;
        let result = day18a(&maze);
        assert_eq!(result, 86);
        Ok(())
    }

    #[test]
    fn test_18_3() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX3.parse()?;
        let result = day18a(&maze);
        assert_eq!(result, 132);
        Ok(())
    }

    #[test]
    fn test_18_4() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX4.parse()?;
        let result = day18a(&maze);
        assert_eq!(result, 136);
        Ok(())
    }

    #[test]
    fn test_18_5() -> Result<(), Box<dyn std::error::Error>> {
        let maze = EX5.parse()?;
        let result = day18a(&maze);
        assert_eq!(result, 81);
        Ok(())
    }

    #[test]
    fn test_18() -> Result<(), Box<dyn std::error::Error>> {
        let maze = crate::util::get_parsed::<Maze>("input/day18.txt")?;
        let part1 = day18a(&maze);
        assert_eq!(part1, 5068);
        Ok(())
    }
}
