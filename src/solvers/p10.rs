use crate::utility;

pub fn p10() {
    let mut sum:i64 = 2;
    let mut current:i64 = 3;

    while current < 2_000_000 {
        if utility::is_prime(current) {
            sum += current;
        }

        current += 2;
    }

    println!("{sum}");
}
