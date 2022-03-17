fn main() {
    let num: u64 = 600851475143;
    let sqrt = (num as f64).sqrt().floor() as u64;
    for i in 2..=sqrt {
        if num % i == 0 && is_prime(i) {
            println!("Prime divider: {}", i);
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
