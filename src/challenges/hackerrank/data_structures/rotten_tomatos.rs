use std::collections::VecDeque;

pub fn run() -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![
        vec![
            2, 1, 1
        ],
        vec![
            1, 1, 0
        ],
        vec![
            0,1,1
        ],
    ];
    let rows = grid.len();
    let columns = grid[0].len();
    let mut fresh = 0;
    let mut minutes = 0;
    let mut rotten: VecDeque<(usize, usize)> = VecDeque::new();


    for r in 0..rows {
        for c in 0..columns {
            if grid[r][c] == 2 {
                rotten.push_back((r, c))
            } else if grid[r][c] == 1 {
                fresh += 1;
            }
        }
    }

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    while fresh > 0 && !rotten.is_empty() {
        minutes += 1;
        let rotten_fruits = rotten.len();
        for _ in 0..rotten_fruits {
            let (r, c) = rotten.pop_front().unwrap();
            for (dr, dc) in directions.iter() {
                let (cr, cc) = (r as isize - dr, c as isize - dc);
                if cr >= 0 && cr < rows as isize && cc >= 0 && cc < columns as isize && grid[cr as usize][cc as usize] == 1 {
                    grid[cr as usize][cc as usize] = 2;
                    fresh -= 1;
                    rotten.push_back((cr as usize, cc as usize))
                }
            }
        }
    }

    if fresh == 0 { minutes } else { -1 }
}