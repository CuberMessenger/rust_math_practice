const GRID_SIZE: usize = 20 * 2 + 1;

fn find_lp(row: usize, column: usize, lp: &[[i64; GRID_SIZE]; GRID_SIZE]) -> i64 {
    if lp[row][column] > 0 {
        return lp[row][column];
    }

    let mut result: i64 = 0;

    if row > 0 {
        result += find_lp(row - 1, column, lp);
    }

    if column > 0 {
        result += find_lp(row, column - 1, lp);
    }

    return result;
}

pub fn p15() {
    let mut lp: [[i64; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    lp[0][0] = 1;

    for num_turn in 1..=(GRID_SIZE - 1) {
        for row in 0..=(num_turn / 2) {
            let column: usize = num_turn - row;

            let value: i64 = find_lp(row, column, &lp);

            lp[row][column] = value;
            lp[column][row] = value;

            println!("{}-{}={}!", row, column, value);
        }

        println!("");
    }

    println!("{}", lp[20][20]);
}

