
// O(sqrt(n)) is prime
pub fn is_prime(num: i64) -> bool {
    if num == 1 {
        return false;
    }

    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    let mut factor: i64 = 3;
    let square_root: i64 = (num as f64).sqrt().ceil() as i64;

    while factor <= square_root {
        if num % factor == 0 {
            return false;
        }

        factor += 2;
    }

    return true;
}

// Large integer
// digits: Vec<u8>
// num = digits[0] + digits[1] * 10 + ...

pub fn from_integer(n: i64) -> Vec<u8> {
    let mut current: i64 = n;
    let mut digits: Vec<u8> = Vec::new();

    while current > 0 {
        digits.push((current % 10) as u8);
        current /= 10;
    }

    return digits;
}

pub fn to_string(digits: &Vec<u8>) -> String {
    let mut cs: Vec<char> = Vec::new();
    
    for digit in digits {
        cs.push((b'0' + digit) as char);
    }

    cs.reverse();

    return cs.iter().collect();
}

pub fn large_integer_add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut carry: u8 = 0;
    let mut digits: Vec<u8> = Vec::new();

    for i in 0..(std::cmp::max(a.len(), b.len())) {
        let mut sum: u8 = carry;
        
        if i < a.len() {
            sum += a[i];
        }

        if i < b.len() {
            sum += b[i];
        }
        
        if sum >= 10 {
            carry = 1;
            sum -= 10;
        }
        else {
            carry = 0;
        }

        digits.push(sum);
    }

    if carry == 1 {
        digits.push(1);
    }

    return digits;
}

pub fn large_integer_multiply(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();

    for i in 0..a.len() {
        let mut current: Vec<u8> = Vec::new();

        let mut carry: u8 = 0;

        for j in 0..b.len() {
            let mut digit = carry;
            digit += a[i] * b[j];

            current.push(digit % 10);
            carry = digit / 10;
        }

        if carry > 0 {
            current.push(carry);
        }

        for _ in 0..i {
            current.insert(0, 0);
        }

        digits = large_integer_add(&digits, &current);
    }

    return digits;
}
