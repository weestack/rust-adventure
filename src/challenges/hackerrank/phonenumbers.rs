use std::collections::HashMap;
use std::ops::Index;

enum PhoneNumbers {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl PhoneNumbers {
    pub fn possible_letters(num: char) -> Vec<char> {
        match num {
            '0' => vec![],
            '1' => vec![],
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => panic!("unknown number given")
        }
    }
}

pub fn run(digits: String)  -> Vec<String> {
    //permutations_of_a_string();
    let mut chars: Vec<char> = digits.chars().collect();
    let mut results: HashMap<String, usize> = HashMap::new();
    let mut target: Vec<char> = Vec::with_capacity(chars.len());
    let mut target_len = digits.len();
    backtrack(&mut chars,
              &mut target,
              &mut results,
              0,
              &mut target_len
    );
    results.keys().cloned().collect::<Vec<String>>()
}



pub fn backtrack(
    chars: &mut Vec<char>,
    target: &mut Vec<char>,
    results: &mut HashMap<String, usize>,
    idx: usize,
    target_len: &mut usize
) {
    if target.len() == *target_len {
        results.insert(target.iter().collect(), 0);
        return;
    }

    let digit = chars.index(idx);
    let letters = PhoneNumbers::possible_letters(*digit);

    for ch in letters {
        target.push(ch);
        backtrack(
            chars,
            target,
            results,
            idx +1,
            target_len
        );
        target.pop().unwrap();
    }
}

fn permutations_of_a_string() {
    let string = "ABC".chars().collect();
    let mut results: HashMap<String, usize> = HashMap::new();
    permutations(
        string,
        &mut results,
        0,
    );
    println!("Results {:?}", results.keys());
}

fn permutations(
    mut input: Vec<char>,
    results: &mut HashMap<String, usize>,
    char_pos: usize
) {
    if char_pos == input.len() {
        println!("{:?}", input.iter().collect::<String>());
        results.insert(input.iter().collect(), 0);
        return;
    }

    for i in 0..input.len() {
        input.swap(char_pos, i);
        permutations(
            input.clone(),
            results,
            char_pos +1
        );
        input.swap(char_pos, i)
    }
}