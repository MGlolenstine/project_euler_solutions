fn main() {
    let mut number = 1;
    for i in 2..u32::MAX {
        let num = number_of_factors(number);
        if num > 500 {
            println!("Number is {number}");
            break;
        }
        number += i;
    }
}

fn number_of_factors(input: u32) -> u32 {
    let mut counter = 0;
    let sqrt = (input as f64).sqrt().floor() as u32;
    for i in 1..sqrt{
        if input % i == 0{
            if input/i == i{
                counter+=1;
            }else{
                counter+=2;
            }
        }
    }
    counter
}
