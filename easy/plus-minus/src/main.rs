use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for i in arr {
        if i > &0 {
            positive += 1;
        } else if i < &0 {
            negative += 1;
        } else {
            zero += 1;
        }
    }

    println!("Positive ratio: {}", positive as f64 / arr.len() as f64);
    println!("Negative ratio: {}", negative as f64 / arr.len() as f64);
    println!("Zero ratio: {}", zero as f64 / arr.len() as f64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Input the size of the array: ");
    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Input the array by space separated integers: ");
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
