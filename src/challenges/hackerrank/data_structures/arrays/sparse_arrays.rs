use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};


fn matchingStrings(stringList: &[String], queries: &[String]) -> Vec<i32> {
    // Write your code here
    let mut matches: Vec<i32> = Vec::new();
    
    for query in queries {
        //println!("============");
        matches.push(stringList.iter().filter(|word| *word == query).count() as i32);
        /*let mut query_match: i32 = 0;
        let q_len = query.len();
        //println!("q: {query:?}");
        for string in stringList {
            //println!("match: {string:?}");
            // Cannot query in smaller string
            let s_len = string.len();
            if string == query {
                query_match += 1;
            }
            if q_len > s_len { continue }
            /*for i in 0..(s_len - q_len) +1 {
                let part = &string[i..i+q_len];
                println!("{:?} - {:?}", *query, part);
                if string[i..i+q_len] == *query {
                    query_match += 1;
                }
            }*/
        }
        matches.push(query_match);*/
    }
    matches
}

pub fn run() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let stringList_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut stringList: Vec<String> = Vec::with_capacity(stringList_count as usize);

    for _ in 0..stringList_count {
        let stringList_item = stdin_iterator.next().unwrap().unwrap();
        stringList.push(stringList_item);
    }

    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    println!("{stringList:?}");
    let res = matchingStrings(&stringList, &queries);
    println!("res: {res:?}");
    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}