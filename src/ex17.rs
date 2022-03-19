fn main() {
    let mut sum = 0;
    for i in 1..=1000{
        println!("{}: {}", i, name_from_number(i));
        let num = text_len(&name_from_number(i));
        sum += num;
    }
    println!("{sum}");
}

fn text_len(text: &str) -> usize{
    text.chars().filter(|&a| a != '-' && a != ' ').count()
}

fn name_from_number(num: u32) -> String {
    let name = if num == 1000 {
        "one thousand".to_string()
    } else if num >= 100 {
        let hundred = num / 100;
        if num - 100 * hundred != 0 {
            format!(
                "{} hundred and {}",
                name_from_number(num/100),
                name_from_number(num - 100 * hundred)
            )
        } else {
            format!("{} hundred", name_from_number(num/100))
        }
    } else if num < 100 && num >= 21 {
        if num % 10 != 0{
            match num / 10 {
                2 => format!("twenty-{}", name_from_number(num%10)),
                3 => format!("thirty-{}", name_from_number(num%10)),
                4 => format!("forty-{}", name_from_number(num%10)),
                5 => format!("fifty-{}", name_from_number(num%10)),
                8 => format!("eighty-{}", name_from_number(num%10)),
                6 | 7 | 9 => format!("{}ty-{}", name_from_number(num/10), name_from_number(num%10)),
                _=>{unimplemented!()}
            }
        }else{
            match num / 10 {
                2 => format!("twenty"),
                3 => format!("thirty"),
                4 => format!("forty"),
                5 => format!("fifty"),
                8 => format!("eighty"),
                6 | 7 | 9 => format!("{}ty", name_from_number(num/10)),
                _=>{unimplemented!()}
            }
        }
    } else if num < 21 {
        match num {
            0 => "zero".to_string(),
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            18 => "eighteen".to_string(),
            16 | 17 | 19 => format!("{}teen", name_from_number(num % 10)),
            20 => "twenty".to_string(),
            _=>{unimplemented!()}
        }
    } else {
        panic!("Owo");
    };
    name.to_string()
}
