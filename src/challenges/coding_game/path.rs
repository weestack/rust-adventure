use std::{collections::HashMap, io};
use fun_stuff::parse_input;

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    visited_spaces: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(map: Vec<Vec<char>>, map_width: usize, map_height: usize) -> Self {
        Map {
            grid: map,
            visited_spaces: HashMap::new(),
            width: map_width - 1,
            height: map_height - 1,
        }
    }

    fn locate_treasure_from(&mut self, starting_row: usize, starting_column: usize) -> usize {
        let mut steps_to_treasure: usize = 1;
        let (mut cr, mut cc) = (starting_row, starting_column);

        loop {
            if cr > self.height || cc > self.width || self.clone().has_visisted_space(cr, cc) {
                // Trap
                return usize::MAX;
            }
            self.visited_spaces.insert((cr, cc), self.grid[cr][cc]);
            let next_move = Direction::parse_direction(self.grid[cr][cc]);

            if let Some(mov) = next_move {
                match mov {
                    Direction::UP => cr -= 1,
                    Direction::LEFT => cc -= 1,
                    Direction::RIGHT => cc += 1,
                    Direction::DOWN => cr += 1,
                }
            } else {
                // . | # | T
                break
            }

            steps_to_treasure += 1;
        }
        if self.grid[cr][cc] != 'T' {
            return usize::MAX
        }

        steps_to_treasure
    }

    fn has_visisted_space(self, current_row: usize, currenct_column: usize) -> bool {
        self.visited_spaces
            .iter()
            .any(|item| item.0.0 == current_row && item.0.1 == currenct_column)
    }
}

#[derive(Debug)]
struct TreasureQuest {
    maps: Vec<Map>,
    easiest_map: usize,
    easiest_map_distance: usize,
}

impl TreasureQuest {
    fn new() -> Self {
        TreasureQuest {
            maps: Vec::new(),
            easiest_map: usize::MAX,
            easiest_map_distance: usize::MAX,
        }
    }

    fn add_map(&mut self, map: Map) {
        self.maps.push(map);
    }

    fn find_treasures(mut self, starting_row: usize, starting_column: usize) -> usize {
        for (map_idx, map) in self.maps.iter_mut().enumerate() {
            let result = map.locate_treasure_from(starting_row, starting_column);
            println!("{map_idx:?} {result}");
            if result < self.easiest_map_distance {
                self.easiest_map_distance = result;
                self.easiest_map = map_idx;
            }
        }
        self.easiest_map
    }
}

enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl Direction {
    fn parse_direction(mov: char) -> Option<Direction> {
        match mov {
            '^' => Some(Direction::UP),
            '<' => Some(Direction::LEFT),
            '>' => Some(Direction::RIGHT),
            'v' => Some(Direction::DOWN),
            _ => None,
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
pub(crate) fn run() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let start_row = parse_input!(inputs[0], usize);
    let start_col = parse_input!(inputs[1], usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut treasure_quest = TreasureQuest::new();

    for _ in 0..n as usize {
        let mut current_map = Vec::new();
        for _ in 0..h as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let map_row = input_line.trim_matches('\n').to_string();
            current_map.push(map_row.chars().collect());
        }
        treasure_quest.add_map(
            Map::new(
                current_map,
                w,
                h
            )
        );
    }

    let best = treasure_quest.find_treasures(start_row, start_col);

    if best == usize::MAX {
        println!("TRAP");
    } else {
        println!("{best:?}");
    }
}
