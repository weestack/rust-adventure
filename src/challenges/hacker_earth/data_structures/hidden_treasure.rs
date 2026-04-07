use std::collections::HashMap;
use std::io;

/// https://www.hackerearth.com/practice/data-structures/arrays/1-d/practice-problems/algorithm/count-valid-pairs-2-da02a8f2/
pub fn run() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut pairs: HashMap<u64, u64> = HashMap::new();
    for num in line.split_whitespace() {
        let mut sum: u64 = 0;
        for byte in num.bytes() {
            sum += match byte.checked_sub(b'0') {
                None => panic!("error"),
                Some(digit) => digit as u64
            }
        }
        pairs.entry(sum).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut total_pairs = 0u64;
    // combinatorics sum f * (f - 1) / 2
    for &count in pairs.values() {
        if count > 1 {
            total_pairs += count * (count -1) / 2
        }
    }

    println!("{}", total_pairs)
}