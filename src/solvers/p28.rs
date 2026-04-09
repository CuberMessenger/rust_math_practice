fn next_direction(direction: (isize, isize)) -> (isize, isize) {
    if direction == (1, 0) {
        return (0, 1);
    } else if direction == (0, 1) {
        return (-1, 0);
    } else if direction == (-1, 0) {
        return (0, -1);
    } else if direction == (0, -1) {
        return (1, 0);
    } else {
        return (0, 0);
    }
}

pub fn p28() {
    const N: usize = 1001;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; N]; N];
    // clock: 1 2 3 4 5 6 7 8...
    // steps: 1 1 2 2 3 3 4 4 ...
    // directions: (1, 0) ... (0, 1) ... (-1, 0) ... (0, -1) ...

    let mut i: usize = N / 2;
    let mut j: usize = N / 2;
    let mut v: u32 = 1;
    matrix[i][j] = v;

    let mut clock: i32 = 0;
    let mut direction: (isize, isize) = (0, -1);

    fn in_range(index: usize) -> bool {index < N}

    while in_range(i) && in_range(j) {
        clock += 1;
        direction = next_direction(direction);
        let step: i32 = (clock + 1) / 2;
        
        for _ in 0..step {
            v += 1;
            i = i.saturating_add_signed(direction.0);
            j = j.saturating_add_signed(direction.1);

            if in_range(i) && in_range(j) {
                matrix[i][j] = v;
            }
            else {
                break;
            }
        }
    }

    let mut sum: u32 = 0;
    for t in 0..N {
        sum += matrix[t][t];
        sum += matrix[t][N - t - 1];
    }
    sum -= 1;

    println!("{}", sum);
}
