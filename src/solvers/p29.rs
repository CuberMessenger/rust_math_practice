use std::{collections::HashSet};
use crate::utility;

pub fn p29() {
    let mut sequence: HashSet<String> = HashSet::new();
    let n = 100;

    for a in 2..=n {
        for b in 2..=n {
            println!("Processing {}^{}!", a, b);
            
            let large_a = utility::from_integer(a);
            
            let power = utility::large_integer_power(&large_a, b as u64);

            let signiture = utility::to_string(&power);

            sequence.insert(signiture);

        }
    }

    println!("{}", sequence.len());

}