use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn hourglassSum(arr: &[Vec<i32>]) -> i32 {
    let glasses: Vec<Vec<i32>> = Vec::with_capacity(4);
    let mut max = i32::MIN;
    for i in 1..arr.len() -1 {
        let row: Vec<i32>  = Vec::with_capacity(4);
        for j in 1 .. arr[0].len() -1 {
            /* a b c
             *   d
             * e f g
             */
            let a = arr[i-1][j-1];
            let b = arr[i-1][j];
            let c = arr[i-1][j+1];
            let d = arr[i][j];
            let e = arr[i+1][j-1];
            let f = arr[i+1][j];
            let g = arr[i+1][j+1];
            let sum = a + b + c + d + e + f + g;
            if sum > max {
                max = sum;
            }
        }
    }
    max
}

pub fn run() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(6_usize);

    for i in 0..6_usize {
        arr.push(Vec::with_capacity(6_usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = hourglassSum(&arr);
    println!("result {result}")
    //writeln!(&mut fptr, "{}", result).ok();
}
