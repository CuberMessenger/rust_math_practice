fn is_palindrome(num: i64) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();

    let length = chars.len();

    for i in 0..(length >> 1) {
        if chars[i] != chars[length - 1 - i] {
            return false;
        }
    }

    return true;
}

pub fn p4() {

    let summit = 1000;

    /*
        A one-digit example:
        layer 2: (l, r) = (1, 1); products = (10 - 1) * (10 - 1)
        layer 3: (l, r) = (1, 2); products = (10 - 1) * (10 - 2)
        layer 4: (l, r) = (2, 2), (1, 3); products = (10 - 2) * (10 - 2), (10 - 1) * (10 - 3)
        ...

        In this order, the product value always decreases.
     */

    for layer in 2..(summit / 2 + 1) {
        let mut l = layer / 2;
        let mut r = layer - l;

        while l > 1 {
            let product = (summit - l) * (summit - r);
            if is_palindrome(product) {
                println!("{product}");
                return;
            }

            l -= 1;
            r += 1;
        }
    }
}
