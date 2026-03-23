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

    const N: usize = 28123; // 4179871
    // const N: usize = 50000; // 4179871
    // const N: usize = 100;
    const FIRST_ABUNDENT: usize = 12;

    let mut is_abundant: [bool; N + 1] = [false; N + 1];
    let mut is_abundant_sums: [bool; N + 1] = [false; N + 1];

    for i in FIRST_ABUNDENT..=N {
        is_abundant[i] = sum_proper_divisors(i as u32) > (i as u32);
        
        if is_abundant[i] {
            // println!("{} is an abundent number!", i);
        }
    }

    for i in FIRST_ABUNDENT..=N {
        if is_abundant[i] {
            for j in FIRST_ABUNDENT..=i {
                if is_abundant[j] && (i + j <= N){
                    is_abundant_sums[i + j] = true;
                }
            }
        }
    }

    let mut abundant_sums_sum: u32 = 0;
    for i in 1..=N {
        if !is_abundant_sums[i] {
            abundant_sums_sum += i as u32;
            // println!("{} cannot be writen as the sum of two abundent numbers!", i);
        }
    }    
    
    println!("{}", abundant_sums_sum);

}

