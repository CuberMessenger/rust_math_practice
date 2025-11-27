fn get_num_divisor(num:i64) -> i64 {
    if num <= 2 {
        return num;
    }

    let mut num_divisor:i64 = 0;
    let mut factor: i64 = 3;
    let square_root: i64 = (num as f64).sqrt().ceil() as i64;

    while factor <= square_root {
        if num % factor == 0 {
            if factor * factor == num {
                num_divisor += 1;
            }
            else {
                num_divisor += 2;
            }
        }

        factor += 1;
    }

    return num_divisor;
}

pub fn p12() {
    let mut natural_num:i64 = 7;
    let mut tri_num:i64 = 28;
    let mut num_divisor:i64 = 6;

    while num_divisor <= 500 {
        natural_num += 1;
        tri_num += natural_num;

        num_divisor = get_num_divisor(tri_num);
    }

    println!("{tri_num}");
}

