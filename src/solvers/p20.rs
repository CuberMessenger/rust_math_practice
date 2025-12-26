use crate::utility;

pub fn p20() {
    let n: i64 = 100;
    let mut p: Vec<u8> = utility::from_integer(1);

    for factor in 2..=n {
        let factor_large: Vec<u8> = utility::from_integer(factor);
        p = utility::large_integer_multiply(&p, &factor_large);

        println!("{}! = {}", factor, utility::to_string(&p));
    }

    let mut sum: i64 = 0;

    for d in p {
        sum += d as i64;
    }

    println!("{}", sum);
}
