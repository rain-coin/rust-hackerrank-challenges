use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    
    let mut stdin_iterator = stdin.lock().lines();

    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let output_path = exe_dir.join(env::var("OUTPUT_PATH").unwrap_or_else(|_| "a-very-big-sum.txt".to_string()));

    let mut fptr = File::create(&output_path).unwrap();

    println!("Input the count of the vector: ");

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Input vector by space separated integers: ");

    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = a_very_big_sum(&ar);

    println!("Sum of the vector: {}", result);

    writeln!(&mut fptr, "{}", result).ok();

}
