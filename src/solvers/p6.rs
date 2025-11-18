fn sum_of_squares(n:i64) -> i64 { (n * (n + 1) * (n + n + 1)) / 6 }

fn square_of_sum(n:i64) -> i64 { (n * n * (n + 1) * (n + 1)) / 4  }

pub fn p6() {
    let n:i64 = 100;
    let difference:i64 = square_of_sum(n) - sum_of_squares(n);

    println!("{difference}");
}
