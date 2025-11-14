use std::cmp;

mod utility;

fn p1() {
    let mut sum: i32 = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("{sum}");
}

fn p2() {
    let mut sum: i32 = 0;

    let mut a: i32 = 1;
    let mut b: i32 = 2;

    while b < 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }

        (a, b) = (b, a + b);
    }

    println!("{sum}");
}

fn p3() {
    let mut num: i64 = 600851475143;

    let mut largest_prime_factor: i64 = -1;

    let square_root: i64 = (num as f64).sqrt().ceil() as i64;

    let mut factor: i64 = 3;

    while factor <= square_root {
        while num % factor == 0 {
            num /= factor;
            largest_prime_factor = cmp::max(largest_prime_factor, factor);
        }

        factor += 2;
    }

    print!("{largest_prime_factor}");
}


fn main() {
    p1();
    p2();
    p3();
}
