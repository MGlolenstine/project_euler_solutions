fn main() {
    //? 0 is Monday
    let mut starting_day = 1;
    let mut sunday_count = 0;
    for i in 1901..2001 {
        let leap = is_leap(i);
        for m in 1..=12 {
            starting_day = (days_in_month(leap, m) + starting_day) % 7;
            if starting_day == 6 {
                sunday_count += 1;
            }
        }
    }
    println!("{sunday_count}");
}

fn days_in_month(leap: bool, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        _ => unimplemented!(),
    }
}

fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
