use std::fs;

fn char_to_worth(c: char) -> u32 {(c.to_ascii_lowercase() as u32) - ('a' as u32) + 1}

pub fn p22() {
    let mut total_score: u32 = 0;
    let raw_names = fs::read_to_string("assets/p22_names.txt").expect("Read failed");
    let mut names:Vec<&str> = raw_names.split(',').collect();
    names.sort();

    
    let mut i: u32 = 1;
    for name_string in names {
        let name_chars: Vec<char> = name_string.chars().collect();

        let mut name_worth: u32 = 0;

        for c in name_chars {
            if c.is_alphabetic() {
                name_worth += char_to_worth(c);
            }
        }

        let name_score: u32 = name_worth * i;
        total_score += name_score;

        // println!("name {} is worthing {} scores!", name_string, name_score);

        i += 1;
    }

    println!("{}", total_score);
}
