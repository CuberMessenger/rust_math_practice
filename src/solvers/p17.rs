
pub fn p17() {
    let mut num_to_length: [i32; 1001] = [0; 1001];

    // dummy
    num_to_length[0] = 4; // zero

    //// 1 to 100
    // 1 to 10
    num_to_length[1] = 3; // one
    num_to_length[2] = 3; // two
    num_to_length[3] = 5; // three
    num_to_length[4] = 4; // four
    num_to_length[5] = 4; // five
    num_to_length[6] = 3; // six
    num_to_length[7] = 5; // seven
    num_to_length[8] = 5; // eight
    num_to_length[9] = 4; // nine
    num_to_length[10] = 3; // ten

    // 11 to 19
    num_to_length[11] = 6; // eleven
    num_to_length[12] = 6; // twelve
    num_to_length[13] = 8; // thirteen
    num_to_length[14] = 8; // fourteen
    num_to_length[15] = 7; // fifteen
    num_to_length[16] = 7; // sixteen
    num_to_length[17] = 9; // seventeen
    num_to_length[18] = 8; // eighteen
    num_to_length[19] = 8; // nineteen

    // 20, 30, ..., 90
    num_to_length[20] = 6; // twenty
    num_to_length[30] = 6; // thirty
    num_to_length[40] = 5; // forty
    num_to_length[50] = 5; // fifty
    num_to_length[60] = 5; // sixty
    num_to_length[70] = 7; // seventy
    num_to_length[80] = 6; // eighty
    num_to_length[90] = 6; // ninety

    // rest
    for t in 2..=9 {
        for d in 1..=9 {
            // <t * 10> <d>
            num_to_length[t * 10 + d] = num_to_length[t * 10] + num_to_length[d];
        }
    }

    //// 100 to 1000
    // 100, 200, ..., 900
    for h in 1..=9 {
        // <h> hundred
        num_to_length[h * 100] = num_to_length[h] + 7;
    }

    // 1000
    num_to_length[1000] = 11; // one thousand

    // rest
    for h in 1..=9 {
        for td in 1..=99 {
            // <h> hundred and <td>
            num_to_length[h * 100 + td] = num_to_length[h] + 7 + 3 + num_to_length[td];
        }
    }

    let mut sum: i32 = 0;
    for i in 1..=1000 {
        println!("num_to_length[{}] = {}", i, num_to_length[i]);
        sum += num_to_length[i];
    }

    println!("{sum}");
}
