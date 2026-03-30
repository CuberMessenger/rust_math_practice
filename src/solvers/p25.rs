use crate::utility;



pub fn p25() {

    let mut F: Vec<Vec<u8>> = Vec::new();


    F.push(utility::from_integer(1 as i64));
    F.push(utility::from_integer(1 as i64));

    let mut num_digits: usize = 1;
    let mut index: u64 = 2;

    while num_digits < 1000 {
        let length = F.len();

        F.push(
            utility::large_integer_add(&F[length - 2], &F[length - 1])
        );

        num_digits = F[length].len();

        index += 1;

        println!("index: {}, num_digits: {}, number: {}", index, num_digits, utility::to_string(&F[length]));
    }

    println!("{}", index);
}
