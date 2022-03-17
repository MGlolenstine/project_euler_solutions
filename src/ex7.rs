fn main() {
    let nth_prime = 10_001;
    let mut counter = 0;
    for i in 2..u64::MAX {
        if is_prime(i) {
            counter += 1;
            if counter == nth_prime {
                println!("{i}");
                break;
            }
        }
    }
}

fn is_prime(input: u64) -> bool {
    let sqrt = (input as f64).sqrt().floor() as u64;
    for i in 2..=sqrt {
        if input % i == 0 {
            return false;
        }
    }
    true
}
