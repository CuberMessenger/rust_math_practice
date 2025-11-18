use crate::utility;

pub fn p7() {
    let target_index:i32 = 10001;

    let mut index:i32 = 1;
    let mut num:i64 = 3;

    while index < target_index {
        if utility::is_prime(num) {
            index += 1;
            println!("Found {index}-th prime {num}!");
        }
        num += 2;
    }

    num -= 2;

    println!("{num}");
}

