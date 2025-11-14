pub fn p2() {
    let mut sum: i32 = 0;

    let mut a: i32 = 1;
    let mut b: i32 = 2;

    while b < 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }

        (a, b) = (b, a + b);
    }

    println!("{sum}");
}
