use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::time::Instant;

#[derive(Copy, Clone)]
enum Movement {
    Up,
    Right,
    Left,
    Down
}

impl Movement {

    pub fn is_valid_movement(self, position: &Position, visited: &mut HashMap<Position, usize>, maze: &Vec<Vec<u8>>) -> bool {

        if visited.contains_key(&position) {
            return false
        }

        if maze[position.y][position.x] == 0 {
            return false
        }
        true
    }

    pub fn parse_movement(self) -> char {
        match self {
            Movement::Up => 'U',
            Movement::Right => 'R',
            Movement::Left => 'L',
            Movement::Down => 'D',
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    pub fn get_new_position(mut self, movement: Movement, maze_size: usize) -> Option<Position> {
         match movement {
            Movement::Up => if self.y == 0 { None } else { self.y = self.y-1; Some(self) },
            Movement::Right => if self.x == maze_size { None } else {self.x = self.x+1; Some(self) }
            Movement::Left => if self.x == 0 {None} else { self.x = self.x-1; Some(self) },
            Movement::Down => if self.y == maze_size {None} else {self.y = self.y+1; Some(self) }
        }
    }
}

pub fn run(maze: Vec<Vec<u8>>) {
    let start = Instant::now();
    let mut start_pos = Position {x: 0, y: 0};
    let mut positions: VecDeque<(Position, Vec<char>, HashMap<Position, usize>)> = VecDeque::new();
    let mut visited: HashMap<Position, usize> = HashMap::new();
    positions.push_front((start_pos, Vec::new(), visited));
    let mut results: Vec<Vec<char>> = Vec::new();
    let target = Position {x: maze.len() -1, y: maze.len()-1};
    bfs(
        positions,
        &maze,
        &mut results,
        &target
    );
    let res: Vec<String> = results.iter().map(|chars| chars.iter().collect::<String>()).collect();
    println!("BFS Took {}us {res:?}", start.elapsed().as_micros());
    let start = Instant::now();
    let mut start_pos = Position {x: 0, y: 0};
    let mut visited: HashMap<Position, usize> = HashMap::new();
    let mut results: Vec<Vec<char>> = Vec::new();
    let mut path: Vec<char> = Vec::new();
    backtrack(
        &mut start_pos,
        &mut visited,
        &maze,
        &mut results,
        &mut path,
        &target
    );
    let res: Vec<String> = results.iter().map(|chars| chars.iter().collect::<String>()).collect();
    println!("Backtrack took {}us {res:?}", start.elapsed().as_micros());

}

pub fn bfs(
    mut positions: VecDeque<(Position, Vec<char>, HashMap<Position, usize>)>,
    maze: &Vec<Vec<u8>>,
    results: &mut Vec<Vec<char>>,
    target: &Position
) {
    // check

    // go over positions
    while !positions.is_empty() {
        // go over movements
        let mut found = false;
        let (position, path, mut visited) = positions.pop_front().unwrap();
        let movements = vec![Movement::Down, Movement::Right, Movement::Left, Movement::Up];
        for mov in movements {
            let maby_position = position.get_new_position(mov, maze.len()-1);
            if found || maby_position.is_none() { continue }

            let new_position = maby_position.unwrap();
            if mov.is_valid_movement(&new_position, &mut visited, maze) {
                visited.insert(new_position, 0);
                let mut new_path = path.clone();
                new_path.push(mov.parse_movement());
                let mut new_visited = visited.clone();
                if new_position == *target {
                    results.push(new_path.clone());
                    found = true;
                    continue
                }
                positions.push_back((new_position, new_path, visited.clone()));
            }
        }
    }
}

pub fn backtrack(
    position: &mut Position,
    visited: &mut HashMap<Position, usize>,
    maze: &Vec<Vec<u8>>,
    results: &mut Vec<Vec<char>>,
    path: &mut Vec<char>,
    target: &Position
) {
    // At the correct position, push the result
    if *position == *target {
        results.push(path.clone());
        return;
    }
    let movements = vec![Movement::Down, Movement::Up, Movement::Right, Movement::Left];
    for movement in movements {
        let new_position = position.get_new_position(movement, maze.len()-1);

        if new_position.is_none() {
            continue
        }
        path.push(movement.parse_movement());
        if movement.is_valid_movement(&new_position.unwrap(), visited, maze) {
            visited.insert(new_position.unwrap().clone(), 0);
            backtrack(
                &mut new_position.unwrap(),
                visited,
                maze,
                results,
                path,
                target
            );
            visited.remove(position);
        }

        path.pop().unwrap();
    }
}