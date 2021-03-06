/*
 * Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
 *
 * 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 *
 * By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
 */

fn main() {
    let mut f: (i64, i64) = (1, 2);
    let mut sum: i64 = 0;
    while f.1 < 4000000 {
        sum = if f.1 % 2 == 0 {
            sum + f.1
        } else {
            sum
        };

        let t = f.0;
        f.0 = f.1;
        f.1 = t + f.1;
    }
    println!("Sum: {}", sum);
}

fn fib(n: i64) -> i64 {
    let mut f: (i64, i64) = (0, 1);
    for _ in 2..n + 1 {
        let t = f.0 + f.1;
        f.0 = f.1;
        f.1 = t;
    }
    f.1
}
