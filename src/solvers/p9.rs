pub fn p9() {
    let n:i32 = 1000;

    let mut found:bool = false;

    for a in 1..=(n - 1 - 1) {
        for b in 1..=(n - a - 1) {
            let c:i32 = n - a - b;

            if a * a + b * b == c * c {
                found = true;

                let result:i32 = a * b * c;
                println!("{result}");
                break;
            }
        }

        if found {
            break;
        }
    }
}
