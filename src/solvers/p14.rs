fn collatz_next(num: i64) -> i64 {
    if num % 2 == 0 {
        return num / 2;
    }
    else {
        return 3 * num + 1;
    }
}

fn collatz_length(num: i64) -> i64 {
    let verbose: bool = false;
    let mut current: i64 = num;
    let mut length: i64 = 1;

    if verbose {
        print!("{current}");
    }

    while current != 1 {
        current = collatz_next(current);
        length += 1;

        if verbose {
            print!(" -> {current}")
        }
    }

    if verbose {
        println!();
    }
    
    return length;
}

pub fn p14() {
    let mut num: i64 = 1000000;

    let mut max_num: i64 = 0;
    let mut max_length: i64 = 0;

    while num >= 1 {
        let length = collatz_length(num);

        if length >= max_length {
            max_num = num;
            max_length = length;
        }

        num -= 1;
    }

    println!("{max_num}");
}
