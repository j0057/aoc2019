//
// struct Coord
//

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Coord(usize, usize);

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

//
// enum Tile
//

#[derive(Clone)]
enum Tile {
    Wall,
    Floor,
    Portal(String)
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "Wall"),
            Self::Floor => write!(f, "Floor"),
            Self::Portal(x) => write!(f, "Portal({})", x)
        }
    }
}

//
// type Grid
//

type Grid = std::collections::BTreeMap<Coord, Tile>;

//
// struct DonutMaze
//

#[derive(Debug)]
pub struct DonutMaze {
    grid: Grid,
}

impl DonutMaze {
    fn new(grid: Grid) -> Self {
        Self {
            grid,
        }
    }

    fn scan_vert(grid: &Vec<Vec<char>>, vert: std::ops::Range<usize>, x: usize, d: fn(usize) -> usize) -> Vec<(Coord, Tile)> {
        vert.filter_map(|y| match grid[y][x] {
                'A'..='Z' if matches!(grid[y][x+1], 'A'..='Z')
                    => Some((Coord(y, d(x)), Tile::Portal([grid[y][x], grid[y][x+1]].iter().collect()))),
                _   => None
            })
            .collect()
    }

    fn scan_horz(grid: &Vec<Vec<char>>, horz: std::ops::Range<usize>, y: usize, d: fn(usize) -> usize) -> Vec<(Coord, Tile)> {
        horz.filter_map(|x| match grid[y][x] {
                'A'..='Z' if matches!(grid[y+1][x], 'A'..='Z')
                    => Some((Coord(d(y), x), Tile::Portal([grid[y][x], grid[y+1][x]].iter().collect()))),
                _   => None
            })
            .collect()
    }
}

impl std::str::FromStr for DonutMaze {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let g: Vec<Vec<char>> = text
            .lines()
            .map(|line| line
                .chars()
                .collect::<Vec<_>>())
            .collect();

        let w: usize = (2..)
            .find(|&i| g[i][i] == ' ')
            .unwrap();

        let p: Grid = []
            .iter()
            .chain(Self::scan_vert(&g, 2..g.len()-2,   0,              |x| x+2).iter())
            .chain(Self::scan_vert(&g, 2..g.len()-2,   g[0].len()-2,   |x| x-1).iter())
            .chain(Self::scan_vert(&g, w..g.len()-w,   w,              |x| x-1).iter())
            .chain(Self::scan_vert(&g, w..g.len()-w,   g[0].len()-w-2, |x| x+2).iter())
            .chain(Self::scan_horz(&g, 2..g.len()-2,   0,              |y| y+2).iter())
            .chain(Self::scan_horz(&g, 2..g.len()-2,   g.len()-2,      |y| y-1).iter())
            .chain(Self::scan_horz(&g, w..g.len()-w,   w,              |y| y-1).iter())
            .chain(Self::scan_horz(&g, w..g.len()-w,   g.len()-w-2,    |y| y+2).iter())
            .cloned()
            .collect();

        let g: Grid = (2..g.len()-2)
            .flat_map(|y| (2..g[0].len()-2).map(move |x| (y, x)))
            .filter_map(|(y, x)| match g[y][x] {
                '.'       => Some(Ok((Coord(y, x), p.get(&Coord(y, x)).cloned().unwrap_or(Tile::Floor)))),
                '#'       => Some(Ok((Coord(y, x), Tile::Wall))),
                'A'..='Z' => None,
                ' '       => None,
                chr       => Some(Err(ParseError::UnknownCharacter(chr)))
            })
            .collect::<Result<Grid, ParseError>>()?;

        Ok(DonutMaze::new(g))
    }
}

impl AsRef<DonutMaze> for DonutMaze {
    fn as_ref(&self) -> &Self {
        self
    }
}

//
// enum ParseError
//

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown character: {0:?}")]
    UnknownCharacter(char),
}

//
// solution
//

pub fn day20a(maze: &DonutMaze) -> u64 {
    0
}

pub fn day20b(maze: &DonutMaze) -> u64 {
    0
}

//
// tests
//

#[cfg(test)]
mod test {
    #[test]
    fn test_20_1() -> Result<(), Box<dyn std::error::Error>> {
        let maze = concat!("         A           \n",
                           "         A           \n",
                           "  #######.#########  \n",
                           "  #######.........#  \n",
                           "  #######.#######.#  \n",
                           "  #######.#######.#  \n",
                           "  #######.#######.#  \n",
                           "  #####  B    ###.#  \n",
                           "BC...##  C    ###.#  \n",
                           "  ##.##       ###.#  \n",
                           "  ##...DE  F  ###.#  \n",
                           "  #####    G  ###.#  \n",
                           "  #########.#####.#  \n",
                           "DE..#######...###.#  \n",
                           "  #.#########.###.#  \n",
                           "FG..#########.....#  \n",
                           "  ###########.#####  \n",
                           "             Z       \n",
                           "             Z       "  ).parse()?;
        let steps = super::day20a(&maze);
        assert_eq!(steps, 23);
        Ok(())
    }

    #[test]
    fn test_20_2() -> Result<(), Box<dyn std::error::Error>> {
        let maze = concat!("                   A               \n",
                           "                   A               \n",
                           "  #################.#############  \n",
                           "  #.#...#...................#.#.#  \n",
                           "  #.#.#.###.###.###.#########.#.#  \n",
                           "  #.#.#.......#...#.....#.#.#...#  \n",
                           "  #.#########.###.#####.#.#.###.#  \n",
                           "  #.............#.#.....#.......#  \n",
                           "  ###.###########.###.#####.#.#.#  \n",
                           "  #.....#        A   C    #.#.#.#  \n",
                           "  #######        S   P    #####.#  \n",
                           "  #.#...#                 #......VT\n",
                           "  #.#.#.#                 #.#####  \n",
                           "  #...#.#               YN....#.#  \n",
                           "  #.###.#                 #####.#  \n",
                           "DI....#.#                 #.....#  \n",
                           "  #####.#                 #.###.#  \n",
                           "ZZ......#               QG....#..AS\n",
                           "  ###.###                 #######  \n",
                           "JO..#.#.#                 #.....#  \n",
                           "  #.#.#.#                 ###.#.#  \n",
                           "  #...#..DI             BU....#..LF\n",
                           "  #####.#                 #.#####  \n",
                           "YN......#               VT..#....QG\n",
                           "  #.###.#                 #.###.#  \n",
                           "  #.#...#                 #.....#  \n",
                           "  ###.###    J L     J    #.#.###  \n",
                           "  #.....#    O F     P    #.#...#  \n",
                           "  #.###.#####.#.#####.#####.###.#  \n",
                           "  #...#.#.#...#.....#.....#.#...#  \n",
                           "  #.#####.###.###.#.#.#########.#  \n",
                           "  #...#.#.....#...#.#.#.#.....#.#  \n",
                           "  #.###.#####.###.###.#.#.#######  \n",
                           "  #.#.........#...#.............#  \n",
                           "  #########.###.###.#############  \n",
                           "           B   J   C               \n",
                           "           U   P   P               "  ).parse()?;
        let steps = super::day20a(&maze);
        assert_eq!(steps, 58);
        Ok(())
    }

    #[test]
    fn test_20() -> Result<(), Box<dyn std::error::Error>> {
        let maze = crate::util::get_parsed("input/day20.txt")?;
        let part1 = super::day20a(&maze);
        assert_eq!(part1, 0);
        Ok(())
    }
}
