
fn sum_digit_fifth_powers(n: u64) -> u64 {
    let mut left = n;
    let mut sum: u64 = 0;

    while left > 0 {
        let digit = left % 10;
        sum += digit.pow(5);
        left /= 10;
    }

    return sum;
}


pub fn p30() {
    const N: u64 = 1_000_000;

    let mut sum: u64 = 0;

    for n in 2..=N {
        if n == sum_digit_fifth_powers(n) {
            sum += n;
            println!("Found {}!", n);
        }
    }

    println!("{}", sum);
}