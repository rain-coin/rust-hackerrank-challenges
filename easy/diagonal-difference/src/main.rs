use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..arr.len() {
        primary_diagonal += arr[i][i];
        secondary_diagonal += arr[i][arr.len() - i - 1];
    }

    (primary_diagonal - secondary_diagonal).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let output_path = exe_dir.join(env::var("OUTPUT_PATH").unwrap_or_else(|_| "diagonal-difference.txt".to_string()));
    
    let mut fptr = File::create(&output_path).unwrap();

    println!("Input the size of the matrix: ");

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Input the matrix by space separated integers: ");

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);

    println!("Diagonal difference: {}", result);

    writeln!(&mut fptr, "{}", result).ok();
}
