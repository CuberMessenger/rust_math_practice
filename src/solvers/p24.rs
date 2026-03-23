pub fn p24() {
    const L: usize = 10;
    const TARGET_INDEX: usize = 1_000_000;
    let mut all_permutations: [Vec<Vec<char>>; L] = Default::default();
    all_permutations[0].push(vec!['0']);
    
    let mut i: usize = 1;


    while i < L {
        let current_char = (('0' as u8) + (i as u8)) as char;
        
        let mut current_permutations: Vec<Vec<char>> = Vec::new();

        for permutation in &all_permutations[i - 1] {
            for location in 0..=i {
                let mut clone_permutation = permutation.clone();
                clone_permutation.insert(location, current_char);
                current_permutations.push(clone_permutation);
            }
        }

        all_permutations[i] = current_permutations;
        i += 1;
    }

    let mut last_perpumation_strings: Vec<String> = Vec::new();

    for permutation in &all_permutations[L - 1] {
        last_perpumation_strings.push(String::from_iter(permutation));
    }

    last_perpumation_strings.sort();

    // for p in last_perpumation_strings {
    //     println!("{}", p);
    // }

    println!("{}", last_perpumation_strings[TARGET_INDEX - 1]);


}
