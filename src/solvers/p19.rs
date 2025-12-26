fn next_day(year: &mut i32, month: &mut i32, day: &mut i32, day_of_week: &mut i32) {
    let num_days: i32;

    if *month == 2 {
        let is_leap: bool;

        if *year % 4 != 0 {
            is_leap = false;
        }
        else if *year % 400 == 0 {
            is_leap = true;
        }
        else if *year % 100 == 0 {
            is_leap = false;
        }
        else {
            is_leap = true;
        }

        if is_leap {
            num_days = 29;
        }
        else {
            num_days = 28;
        }
    }
    else if matches!(month, 4 | 6 | 9 | 11) {
        num_days = 30;
    }
    else {
        num_days = 31;
    }

    // day of week
    *day_of_week += 1;
    if *day_of_week > 7 {
        *day_of_week = 1;
    }

    // day
    *day += 1;
    if *day > num_days {
        *day = 1;
        *month += 1;
    }

    // month
    if *month > 12 {
        *month = 1;
        *year += 1;
    }

    if *day == 1 && *day_of_week == 7 {
        println!("{}.{}.{}: {}", year, month, day, day_of_week);
    }
}

fn is_start(year: i32, month: i32, day: i32) -> bool {
    year == 1901 && month == 1 && day == 1
}

fn is_end(year: i32, month: i32, day: i32) -> bool {
    year == 2000 && month == 12 && day == 31
}

fn is_sun_as_first(day: i32, day_of_week: i32) -> bool {
    day == 1 && day_of_week == 7
}

pub fn p19() {
    let mut year: i32 = 1900;
    let mut month: i32 = 1;
    let mut day: i32 = 1;
    let mut day_of_week: i32 = 1;

    let mut sum: i32 = 0;

    while !is_start(year, month, day) {
        next_day(&mut year, &mut month, &mut day, &mut day_of_week);
    }

    if is_sun_as_first(day, day_of_week) {
        sum += 1;
    }

    while !is_end(year, month, day) {
        next_day(&mut year, &mut month, &mut day, &mut day_of_week);

        if is_sun_as_first(day, day_of_week) {
            sum += 1;
        }
    }

    println!("{}", sum);
}
