use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result : Vec<i32> = vec![0, 0];

    for (index, a_value) in a.iter().enumerate() {
        if a_value > b.get(index).unwrap() {
            result[0] += 1;
        } else if a_value < b.get(index).unwrap() {
            result[1] += 1;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let output_path = exe_dir.join(env::var("OUTPUT_PATH").unwrap_or_else(|_| "output".to_string()));
    
    let mut fptr = File::create(&output_path).unwrap();

    println!("Input vector a by space separated integers: ");
    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    println!("Input vector b by space separated integers: ");
    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compare_triplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
    println!("Output written to {}", output_path.display());
}
