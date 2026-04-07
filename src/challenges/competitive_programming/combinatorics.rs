



pub fn run() {
    let stars = 4;
    let bars = 2;
    println!("{}", get_ways_with_zero(stars, bars));
    println!("{}", get_ways_with_at_least_one(stars, bars));
    println!("combination sum");
    println!("{:?}", combination_sum(vec![2,3,6,7], 7))
}

// Stars and bars
fn combinations(N: u32, R: u32) -> u32 {
    let R = R.min(N-R);
    println!("R {R}");
    let mut ans = 1;
    println!("ans {ans}");
    for i in 0..R {
        ans = ans * (N - i);
        println!("ans {ans} * {N} - {i}");
        ans = ans / (i + 1);
        println!("ans {ans} / {i} - 1");
    }
    ans
}

fn get_ways_with_zero(N: u32, K: u32) -> u32 {
    combinations(N + K -1, K -1)
}

fn get_ways_with_at_least_one(N: u32, K: u32) -> u32 {
    combinations(N -1, K - 1)
}

fn combination_sum(candidates: Vec<i32>, target: i32) {
    let temp: Vec<i32> = Vec::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    search(0, target, candidates, &mut result, temp);
    println!("{result:?}")
}

fn search(idx: usize, target: i32, candidates: Vec<i32>, result: &mut Vec<Vec<i32>>, mut temp: Vec<i32>) {
    if target == 0 {
        result.push(temp);
        return;
    }
    if target < 0 {
        return;
    }
    if idx == candidates.len() {
        return;
    }
    temp.push(candidates[idx]);
    search(idx, target - candidates[idx], candidates.clone(), result, temp.clone());
    temp.pop();
    search(idx + 1, target, candidates, result, temp);
}