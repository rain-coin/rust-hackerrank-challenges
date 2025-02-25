use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn min_max_sum(arr: &[i32]) {
    let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let mut min_sum = i64::MAX;
    let mut max_sum = i64::MIN;

    for &value in arr {
        let current_sum = total_sum - value as i64;
        if current_sum < min_sum {
            min_sum = current_sum;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Enter the array by space: ");

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    min_max_sum(&arr);
}
