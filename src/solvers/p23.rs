fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum: u32 = 1;

    for i in 2..=(n / 2) {
        if n % i == 0 {
            sum += i;
        }
    }

    return sum;
}

pub fn p23() {

    let spd = sum_proper_divisors(12);

    println!("{}", spd);

}

