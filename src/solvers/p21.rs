use std::collections::{HashSet};

fn sum_of_divisors(num: i64) -> i64 {
    let mut sum: i64 = 0;

    let mut divisor: i64 = 1;

    while divisor * divisor <= num {
        if divisor * divisor == num {
            sum += divisor;
            break;
        }
        
        if divisor == 1 {
            sum += divisor;
            divisor += 1;
            continue;
        }
        
        if num % divisor == 0 {
            sum += divisor + num / divisor;
        }

        divisor += 1;
    }

    // println!("d({}) = {}", num, sum);

    return sum;
}

pub fn p21() {
    let limit: i64 = 10000;

    let mut amicables: HashSet<i64> = HashSet::new();

    for phobos in 1..limit {
        let deimos: i64 = sum_of_divisors(phobos);

        if phobos != deimos && sum_of_divisors(deimos) == phobos{
            amicables.insert(phobos);
            amicables.insert(deimos);
            println!("d({}) = {}, d({}) = {}", phobos, deimos, deimos, phobos);
        }
    }

    println!();

    let mut sum: i64 = 0;
    for amicable in amicables {
        sum += amicable;
    }

    println!("{}", sum);
}

