
// O(sqrt(n)) is prime
pub fn is_prime(num: i64) -> bool {
    if num == 1 {
        return false;
    }

    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    let mut factor: i64 = 3;
    let square_root: i64 = (num as f64).sqrt().ceil() as i64;

    while factor <= square_root {
        if num % factor == 0 {
            return false;
        }

        factor += 2;
    }

    return true;
}


