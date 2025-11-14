use std::cmp;

pub fn p3() {
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
