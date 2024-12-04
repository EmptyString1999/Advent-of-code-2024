use std::any::Any;
use std::array;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

fn distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut distance: Vec<i32> = Vec::new();

    for (l, r ) in left.iter().zip(right.iter()) {
        if l < r {
            distance.push(r - l);
        } else {
            distance.push(l - r);
        }
    }

    let mut sum: i32 = 0;
    for i in distance.iter() {
        sum += i;
    }

    return sum;
}

fn similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut similarity: HashMap<i32, i32> = HashMap::new();

    for l in left.iter() {
        for r in right.iter() {
            if l == r {
                if similarity.contains_key(l) {
                    let value = similarity.get(l).unwrap();
                    similarity.insert(*l, value + 1);
                } else {
                    similarity.insert(*l, 1);
                }
            }
        }
    }

    let mut sum: i32 = 0;
    for (key, value) in similarity.iter() {
        sum += key * value;
    }

    return sum;
}


fn main() {
    let file = File::open("input.txt").expect("File not found");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new(); 
    // extract left and right 
    let reader = BufReader::new(file);
    let mut count: i32 = 0;
    for line in reader.lines() {
        count += 1;
        let line = line.unwrap();
        let mut parts = line.split("   ");
        left.push(parts.next().unwrap().parse::<i32>().expect("Not a number"));
        right.push(parts.next().unwrap().parse::<i32>().expect("Not a number"));
    }

    // sort left and right
    left.sort();
    right.sort();

    println!("The sum of the distances is: {}", distance(&left, &right));
    println!("The number of similarity socre is: {}", similarity(left, right));
}
