fn free_25(d: u32) -> u32 {
    let mut n = d;

    for divisor in [2, 5] {
        while n % divisor == 0 {
            n /= divisor;
        }
    }

    return n;
}

fn compute_cycle_length(d: u32) -> u32 {
    let n: u32 = free_25(d);

    if n == 1 {
        return 0;
    }

    let mut length: u32 = 0;
    
    let base: u32 = 10;
    let mut divider_length: u32 = 1;
    
    while base.pow(divider_length) < n {
        divider_length += 1;
    }

    let mut divider: u32 = base.pow(divider_length) as u32;

    let first_div = divider / n;
    let mut remainder = divider % n;
    
    let mut div: u32 = 0;
    while first_div != div {
        divider = remainder * (base.pow(divider_length) as u32);
        div = divider / n;
        remainder = divider % n;

        length += 1;
    }

    return length;
}

pub fn p26() {
    let mut longest_d: u32 = 0;
    let mut longest_length: u32 = 0;

    for d in 1..1000 {
        let length = compute_cycle_length(d);
        if length > longest_length {
            longest_length = length;
            longest_d = d;
        }
    }

    println!("{}-{}", longest_d, longest_length);

}