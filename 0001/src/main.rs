/*
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn main() {
    let max: i64 = 1000;
    let sum = efficient_solution(max);
    println!("Sum is: {}", sum);
}

fn efficient_solution(max: i64) -> i64 {
    divisible_by(max, 3) + divisible_by(max, 5) - divisible_by(max, 5 * 3)
}

fn divisible_by(max: i64, n: i64) -> i64 {
    let p: i64 = (max - 1) / n;
    n * (p * (p + 1)) / 2
}

fn simple_solution(max: i64) -> i64 {
    let mut n: i64 = 0;
    let mut sum: i64 = 0;
    while n < max {
        sum = sum + sum_if_multiple(n);
        n = n + 1;
    }
    sum
}

fn sum_if_multiple(n: i64) -> i64 {
    if n % 3 == 0 || n % 5 == 0 {
        n
    } else {
        0
    }
}
