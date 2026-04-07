use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize
}
pub fn run(n: i32) -> Vec<Vec<String>> {
    if n == 1 {
        return vec![vec!['Q'.to_string()]]
    }

    let mut queens: HashSet<Position> = HashSet::new();
    let mut results: Vec<HashSet<Position>> = Vec::new();
    backtrack(
        0,
        n as isize,
        &mut queens,
        &mut results,
    );
    let mut grid: Vec<Vec<String>> = Vec::new();

    for board in results {
        let mut row_solution: Vec<String> = Vec::new();
        for row in 0..n {
            let mut string_row = vec!['.'; n as usize];

            for queen in &board {
                if queen.y == row as isize {
                    string_row[queen.x as usize] = 'Q';
                    break;
                }
            }
            row_solution.push(string_row.iter().collect());
        }
        grid.push(row_solution)
    }

    grid
}

fn backtrack(
    row_idx: isize,
    target: isize,
    queens: &mut HashSet<Position>,
    result: &mut Vec<HashSet<Position>>
) {
    if target == row_idx {
        result.push(queens.clone());
        return;
    }

    for c in 0..target {
        let possible_queen = Position {x: c, y: row_idx};
        let mut allowed_placement = true;
        for queen in queens.iter() {
            let dx = possible_queen.x - queen.x;
            let dy = possible_queen.y - queen.y;
            if dx == 0 || dx.abs() == dy.abs() {
                allowed_placement = false;
                break;
            }
        }

        if !allowed_placement {
            continue
        }
        queens.insert(possible_queen.clone());
        backtrack(
            row_idx +1,
            target,
            queens,
            result,
        );
        queens.remove(&possible_queen);
    }
}