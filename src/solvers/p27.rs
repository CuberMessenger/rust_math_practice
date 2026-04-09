use crate::utility;

fn quadratic(n: i32, a: i32, b: i32) -> i32 { n * n + a * n + b }

fn num_consecutive_primes(a: i32, b: i32) -> u32 {
    let mut count: u32 = 0;
    let mut n: i32 = 0;

    while utility::is_prime(quadratic(n, a, b) as i64) {
        count += 1;
        n += 1;
    }
    
    return count;
}

pub fn p27() {
    let mut longest: u32 = 0;
    let mut product: i32 = 0;

    for a in (-999)..=(999) {
        for b in (-1000)..=(1000) {
            let length = num_consecutive_primes(a, b);
            // if length > 0 {
            //     println!("Find {} consecutive primes for a = {} and b = {}", length, a, b);
            // }
            
            if length >= longest {
                longest = length;
                product = a * b;
                println!("Find {} consecutive primes for a = {} and b = {}", length, a, b);
            }
        }
    }

    println!("{}", product);
}
